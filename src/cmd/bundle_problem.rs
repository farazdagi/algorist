use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::LazyLock;

use prettyplease::unparse;
use quote::ToTokens;
use syn::parse_quote;
use syn::{Item, ItemUse, parse_file, visit::Visit, visit_mut::VisitMut};
use {
    crate::cmd::SubCmd,
    anyhow::{Context, Result, anyhow},
    argh::FromArgs,
    regex::Regex,
    std::{
        fs::{self, File},
        io::{BufRead, BufReader, BufWriter, Write},
        path::PathBuf,
    },
};

/// Bundle given problem into a single file.
#[derive(FromArgs)]
#[argh(subcommand, name = "bundle")]
pub struct BundleProblemSubCmd {
    #[argh(positional)]
    /// problem ID
    id: String,
}

impl SubCmd for BundleProblemSubCmd {
    fn run(&self) -> Result<()> {
        let mut ctx = BundlerContext::new(&self.id).context(format!(
            "failed to create bundler context for problem {}",
            self.id
        ))?;

        Bundler1::new(&mut ctx)?
            .run()
            .context(format!("failed to bundle problem {}", self.id))?;

        Ok(())
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct UsedMod {
    segments: Vec<String>,
}

trait BunlingPhase {}

mod phases {
    use super::*;

    pub struct ProcessBinaryFile {
        pub used_mods: BTreeSet<UsedMod>,
    }

    pub struct ProcessLibraryFile {
        pub used_mods: BTreeSet<UsedMod>,
        pub base_path: PathBuf,
    }

    pub struct BundlingCompleted;

    impl BunlingPhase for ProcessBinaryFile {}
    impl BunlingPhase for ProcessLibraryFile {}
    impl BunlingPhase for BundlingCompleted {}
}

#[derive(Debug)]
struct BundlerContext {
    main_mod: String,
    problem_id: String,
    src: PathBuf,
    dst: PathBuf,
    out: BufWriter<File>,
}

impl BundlerContext {
    fn new(problem_id: &str) -> Result<Self> {
        // Validate the problem ID.
        let src = PathBuf::from(format!("./src/bin/{}.rs", problem_id))
            .canonicalize()
            .context("source file for the problem is not found")?;

        // Create the destination directory if it doesn't exist.
        fs::create_dir_all(PathBuf::from("bundled"))?;
        let dst = PathBuf::from(format!("./bundled/{}.rs", problem_id));
        let out = BufWriter::new(File::create(&dst).context("failed to create output file")?);

        Ok(Self {
            main_mod: MAIN_MOD.to_string(),
            problem_id: problem_id.to_string(),
            src,
            dst,
            out,
        })
    }
}

#[derive(Debug)]
struct Bundler1<'a, P: BunlingPhase = phases::ProcessBinaryFile> {
    ctx: &'a mut BundlerContext,
    state: P,
}

impl<'a> Bundler1<'a, phases::ProcessBinaryFile> {
    fn new(ctx: &'a mut BundlerContext) -> Result<Self> {
        Ok(Self {
            ctx,
            state: phases::ProcessBinaryFile {
                used_mods: BTreeSet::new(),
            },
        })
    }

    fn run(self) -> Result<()> {
        self.process_binary_file()?
            .process_library_file()?
            .complete_bundling()
    }

    fn process_binary_file(mut self) -> Result<Bundler1<'a, phases::ProcessLibraryFile>> {
        let src = self.ctx.src.display().to_string();
        let dst = self.ctx.dst.display().to_string();
        println!("Bundling {src} -> {dst}");

        // Read the executable source file to find used modules.
        let file_content =
            fs::read_to_string(&self.ctx.src).context("failed to read source file")?;
        let mut ast = parse_file(&file_content).context("failed to parse source file")?;
        self.visit_file(&mut ast);

        // Write the source file -- unmodified -- to the output file.
        writeln!(self.ctx.out, "{}", unparse(&ast)).context("failed to write source file")?;

        Ok(Bundler1 {
            ctx: self.ctx,
            state: phases::ProcessLibraryFile {
                used_mods: self.state.used_mods,
                base_path: PathBuf::from("src")
                    .canonicalize()
                    .context("failed to canonicalize src path")?,
            },
        })
    }

    fn process_item_use(&mut self, tree: &syn::UseTree) -> Result<()> {
        let mut segments = Vec::new();

        // Process path segments.
        let mut tree = tree;
        while let syn::UseTree::Path(path) = tree {
            if path.ident == self.ctx.main_mod {
                tree = &*path.tree;
                continue;
            }
            segments.push(path.ident.to_string());
            tree = &*path.tree;
        }

        // Process the final segment, which can be a Name, Rename, Glob or Group.
        match tree {
            syn::UseTree::Name(name) => {
                segments.push(name.ident.to_string());
                self.state.used_mods.insert(UsedMod { segments });
            }
            syn::UseTree::Rename(rename) => {
                segments.push(rename.ident.to_string());
                self.state.used_mods.insert(UsedMod { segments });
            }
            syn::UseTree::Group(group) => {
                for item in &group.items {
                    let mut item_segments = segments.clone();
                    if let syn::UseTree::Name(name) = item {
                        item_segments.push(name.ident.to_string());
                    } else if let syn::UseTree::Rename(rename) = item {
                        item_segments.push(rename.ident.to_string());
                    } else {
                        return Err(anyhow!(
                            "Unexpected UseTree item: {}",
                            item.to_token_stream().to_string()
                        ));
                    }
                    self.state.used_mods.insert(UsedMod {
                        segments: item_segments,
                    });
                }
            }
            syn::UseTree::Glob(_) => {
                // We don't need to do anything here, we already have all segments.
                self.state.used_mods.insert(UsedMod { segments });
            }
            _ => {}
        }

        Ok(())
    }
}

impl<'ast> Visit<'ast> for Bundler1<'_, phases::ProcessBinaryFile> {
    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        // Ignore all imports except those from the current crate.
        if let syn::UseTree::Path(path) = &node.tree {
            if path.ident != self.ctx.main_mod {
                return;
            }
        }

        self.process_item_use(&node.tree)
            .expect("Failed to process use tree");
    }
}

impl<'a> Bundler1<'a, phases::ProcessLibraryFile> {
    fn process_library_file(mut self) -> Result<Bundler1<'a, phases::BundlingCompleted>> {
        // Read the library source file to expand all used modules. Modules are expanded
        // recursively. Modules that are not used in the binary are ignored.
        let file_content =
            fs::read_to_string("src/lib.rs").context("failed to read library file")?;
        let mut ast = parse_file(&file_content).context("failed to parse library file")?;
        self.visit_file_mut(&mut ast);

        // Wrap the items in a module with the main module name.
        let items = std::mem::take(&mut ast.items);
        let mod_item = syn::Item::Mod(syn::ItemMod {
            unsafety: None,
            attrs: vec![
                parse_quote!(#[doc = " [Algorist](https://crates.io/crates/algorist) library"]),
                parse_quote!(#[allow(dead_code)]),
                parse_quote!(#[allow(unused_imports)]),
                parse_quote!(#[allow(unused_macros)]),
            ],
            vis: syn::Visibility::Inherited,
            mod_token: Default::default(),
            ident: syn::Ident::new(&self.ctx.main_mod, proc_macro2::Span::call_site()),
            content: Some((Default::default(), items)),
            semi: None,
        });
        ast.items = vec![mod_item];

        // Write the modified AST back to the output file.
        writeln!(self.ctx.out, "{}", unparse(&ast)).context("failed to write bundled file")?;

        // println!("Code {}", quote::quote!(#ast).to_string());
        println!("Code(unparse)\n{}", unparse(&ast));
        // println!(
        //     "Code(prettify)\n{}",
        //     prettify((&ast).into_token_stream().to_string())
        // );

        Ok(Bundler1 {
            ctx: self.ctx,
            state: phases::BundlingCompleted,
        })
    }

    fn process_item_mod_mut(&mut self, node: &mut syn::ItemMod) {
        // If the module has content, we don't need to do anything.
        if node.content.is_some() {
            return;
        }

        let mod_name = node.ident.to_string();
        println!("Processing root module: {mod_name}");

        // Load the module file from the source directory.
        // Module may be EITHER in the form of `src/foo.rs` or `src/foo/mod.rs`.
        // Try both, and since only one works, we can use `find` to get the first one.
        let (base_path, code): (_, String) = vec![
            format!("{}/{}.rs", self.state.base_path.display(), mod_name),
            format!("{}/{}/mod.rs", self.state.base_path.display(), mod_name),
        ]
        .into_iter()
        .map(PathBuf::from)
        .find(|p| p.exists())
        .map(|p| {
            let base_path = p
                .clone()
                .parent()
                .expect("Failed to get parent directory")
                .to_path_buf();
            (base_path, p)
        })
        .and_then(|(base_path, mod_path)| {
            println!("Loading module file: {:?}", mod_path);
            fs::read_to_string(mod_path)
                .context("failed to read source file")
                .ok()
                .and_then(|code| Some((base_path, code)))
        })
        .expect("Module file not found");

        let mut ast = parse_file(&code)
            .context("failed to parse source file")
            .expect("Failed to parse module file");
        Bundler1 {
            ctx: self.ctx,
            state: phases::ProcessLibraryFile {
                used_mods: self.state.used_mods.clone(),
                base_path,
            },
        }
        .visit_file_mut(&mut ast);

        // Populate the module content with the parsed items.
        node.content = Some((Default::default(), ast.items));
    }
}

impl<'a> VisitMut for Bundler1<'a, phases::ProcessLibraryFile> {
    fn visit_attributes_mut(&mut self, attrs: &mut Vec<syn::Attribute>) {
        // Drop all attributes that are not relevant for bundling.
        *attrs = attrs
            .drain(..)
            .filter(|attr| {
                !attr.path().is_ident("doc")
                    && !attr.path().is_ident("allow")
                    && !attr.path().is_ident("cfg")
                    && !attr.path().is_ident("warn")
            })
            .collect();
    }

    fn visit_file_mut(&mut self, file: &mut syn::File) {
        self.visit_attributes_mut(&mut file.attrs);
        for it in &mut file.items {
            self.visit_item_mut(it);
        }
    }

    fn visit_item_mod_mut(&mut self, node: &mut syn::ItemMod) {
        self.visit_attributes_mut(&mut node.attrs);
        self.visit_visibility_mut(&mut node.vis);
        self.visit_ident_mut(&mut node.ident);

        // process mods
        self.process_item_mod_mut(node);

        if let Some(it) = &mut node.content {
            for it in &mut (it).1 {
                self.visit_item_mut(it);
            }
        }
    }

    fn visit_path_mut(&mut self, path: &mut syn::Path) {
        // dbg!(path);
    }
}

impl<'a> Bundler1<'a, phases::BundlingCompleted> {
    fn complete_bundling(self) -> Result<()> {
        println!(
            "Problem {:?} bundled successfully into {:?}",
            self.ctx.problem_id, self.ctx.dst
        );

        Ok(())
    }
}

const MAIN_MOD: &str = "algorist";

static RE_MOD: LazyLock<Regex> =
    LazyLock::new(|| regex_line(r" (pub  )?mod  (?P<m>.+) ; ").unwrap());
static RE_COMMENT: LazyLock<Regex> = LazyLock::new(|| regex_line(r" ").unwrap());
static RE_WARN: LazyLock<Regex> = LazyLock::new(|| regex_line(r" #!\[warn\(.*").unwrap());
static RE_CFG_TEST: LazyLock<Regex> = LazyLock::new(|| regex_line(r" #\[cfg\(test\)\] ").unwrap());
static RE_ALLOW_DEAD_CODE: LazyLock<Regex> =
    LazyLock::new(|| regex_line(r" #.?\[allow\(dead_code\)\] ").unwrap());
static RE_USE: LazyLock<Regex> =
    LazyLock::new(|| regex_line(format!(" use  {MAIN_MOD}::(?P<submod>.*)::.*$")).unwrap());
static RE_USE_CRATE: LazyLock<Regex> = LazyLock::new(|| {
    regex_line(r" use  (?P<prefix>\{?)crate::(?P<submod>.*)::(?P<postfix>.*)$").unwrap()
});
static RE_MACRO_IMPL_CRATE: LazyLock<Regex> =
    LazyLock::new(|| regex_line(r" impl \$crate::(?P<content>.*) ").unwrap());

struct Bundler {
    src: PathBuf,
    dst: PathBuf,
    out: BufWriter<File>,
    allow: Vec<String>,
}

impl Bundler {
    fn new(src: PathBuf, dst: PathBuf) -> Self {
        let out = BufWriter::new(File::create(&dst).unwrap());
        Self {
            src,
            dst,
            out,
            allow: Vec::new(),
        }
    }

    fn run(&mut self) -> Result<()> {
        let src = self.src.display().to_string();
        let dst = self.dst.display().to_string();
        println!("Bundling {src} -> {dst}");

        self.binrs()?;
        self.librs()?;

        self.out.flush()?;
        self.out.get_ref().sync_all()?;

        std::process::Command::new("rustfmt")
            .arg("+nightly")
            .arg(&self.dst)
            .status()
            .with_context(|| format!("Failed to run rustfmt on {:?}", self.dst))?;

        Ok(())
    }

    fn binrs(&mut self) -> Result<()> {
        let mut reader = BufReader::new(File::open(&self.src)?);
        let mut line = String::new();
        while reader.read_line(&mut line)? > 0 {
            // preserve empty lines
            if line == "\n" {
                self.writeln(&line)?;
                line.clear();
                continue;
            }

            line.pop();
            if is_ignorable(&line) {
                line.clear();
                continue;
            }
            if let Some(caps) = RE_USE.captures(&line) {
                if let Some(m) = caps.name("submod") {
                    self.extend_allow(m.as_str())?;
                }
            }

            self.writeln(&line)?;
            line.clear();
        }
        Ok(())
    }

    fn extend_allow(&mut self, module: &str) -> Result<()> {
        if self.allow.contains(&module.to_string()) {
            return Ok(());
        }

        println!("allow: {module}");

        let reader = mod_reader(module.to_string().replace("::", "/").as_str())?;
        let mut cfg_test_occurred = false;
        let submodules: Vec<String> = reader
            .lines()
            .map_while(Result::ok)
            .filter_map(|l| {
                if RE_CFG_TEST.is_match(&l) {
                    cfg_test_occurred = true;
                }
                if cfg_test_occurred {
                    return None;
                }
                RE_USE_CRATE
                    .captures(&l)
                    .map(|c| c.name("submod").unwrap().as_str().to_string())
            })
            .collect();

        let parts = module.split("::");
        for i in 0..parts.clone().count() {
            let allow = parts.clone().take(i + 1).collect::<Vec<&str>>().join("::");
            if !self.allow.contains(&allow) {
                self.allow.push(allow);
            }
        }

        for m in &submodules {
            self.extend_allow(m).expect("Failed to extend allow list");
        }

        Ok(())
    }

    fn librs(&mut self) -> Result<()> {
        let librs = PathBuf::from("src/lib.rs");
        if !librs.exists() {
            return Err(anyhow!("Error: file not found: {:?}", librs));
        }

        self.writeln("")?;
        self.writeln("#[allow(dead_code)]")?;
        self.writeln("#[allow(unused_imports)]")?;
        self.writeln("#[allow(unused_macros)]")?;
        self.writeln(&format!("mod {MAIN_MOD} {{"))?;

        let mut reader = BufReader::new(File::open(&librs)?);
        let mut line = String::new();
        while reader.read_line(&mut line)? > 0 {
            line.pop();
            if is_ignorable(&line) {
                line.clear();
                continue;
            }
            if let Some(caps) = RE_MOD.captures(&line) {
                if let Some(m) = caps.name("m") {
                    self.modrs(m.as_str(), m.as_str(), 1)?;
                }
            } else {
                self.writeln(&line)?;
            }
            line.clear();
        }

        self.writeln("}")?;

        Ok(())
    }

    fn modrs(&mut self, mod_name: &str, mod_path: &str, lvl: usize) -> Result<()> {
        // Ignore modules that are not used in the binary.
        if !self.allow.contains(&mod_path.replace('/', "::")) {
            println!("ignored module: {mod_name} (path: {mod_path})");
            return Ok(());
        }

        println!(
            "module: {} (path: {})",
            mod_name,
            mod_path.replace('/', "::")
        );

        self.writeln(&format!("pub mod {mod_name} {{"))?;

        let mut reader = mod_reader(mod_path)?;
        let mut line = String::new();
        while reader.read_line(&mut line)? > 0 {
            line.pop();
            if is_ignorable(&line) {
                line.clear();
                continue;
            }
            if RE_CFG_TEST.is_match(&line) {
                break;
            }
            if let Some(caps) = RE_USE_CRATE.captures(&line) {
                if let Some(submod) = caps.name("submod") {
                    let prefix = caps.name("prefix").map_or("", |m| m.as_str());
                    let postfix = caps.name("postfix").map_or("", |m| m.as_str());
                    self.writeln(&format!(
                        "use {}{}{}::{}",
                        prefix,
                        "super::".repeat(lvl),
                        submod.as_str(),
                        postfix
                    ))?;
                }
            } else if let Some(caps) = RE_MACRO_IMPL_CRATE.captures(&line) {
                if let Some(content) = caps.name("content") {
                    println!("macro: {}", content.as_str());
                    self.writeln(&format!("impl crate::{}::{}", MAIN_MOD, content.as_str()))?;
                }
            } else if let Some(caps) = RE_MOD.captures(&line) {
                if let Some(m) = caps.name("m") {
                    self.modrs(m.as_str(), &format!("{}/{}", mod_path, m.as_str()), lvl + 1)?;
                }
            } else {
                self.writeln(&line)?;
            }
            line.clear();
        }

        self.writeln("}")?;

        Ok(())
    }

    fn writeln(&mut self, line: &str) -> Result<()> {
        writeln!(self.out, "{line}").map_err(|e| anyhow!(e))
    }
}

fn is_ignorable(line: &str) -> bool {
    RE_COMMENT.is_match(line) || RE_WARN.is_match(line) || RE_ALLOW_DEAD_CODE.is_match(line)
}

fn mod_reader(mod_path: &str) -> Result<BufReader<File>, anyhow::Error> {
    let reader = [
        format!("src/{mod_path}.rs"),
        format!("src/{mod_path}/mod.rs"),
    ]
    .iter()
    .map(File::open)
    .find(Result::is_ok)
    .ok_or_else(|| {
        anyhow!(
            "Error: file not found: src/{0}.rs or src/{0}/mod.rs",
            mod_path,
        )
    })?
    .map(BufReader::new)?;
    Ok(reader)
}

fn regex_line<S: AsRef<str>>(source_regex: S) -> Result<Regex> {
    Regex::new(
        format!(
            "^{}(?://.*)?$",
            source_regex
                .as_ref()
                .replace("  ", r"\s+")
                .replace(' ', r"\s*")
        )
        .as_str(),
    )
    .map_err(|e| anyhow!(e))
}
