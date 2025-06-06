use {
    crate::cmd::SubCmd,
    anyhow::{Context, Result, anyhow},
    argh::FromArgs,
    lazy_static::lazy_static,
    regex::Regex,
    std::{
        fs::{self, File},
        io::{BufRead, BufReader, BufWriter, Write},
        path::PathBuf,
    },
};

/// Bundle given problem into a single file.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "bundle")]
pub struct BundleSubCmd {
    #[argh(positional)]
    /// problem ID
    id: char,
}

impl SubCmd for BundleSubCmd {
    fn run(&self) -> Result<()> {
        // Validate the problem ID.
        let src = PathBuf::from(format!("./src/bin/{}.rs", self.id))
            .canonicalize()
            .context("failed to canonicalize source path")?;

        // Create the destination directory if it doesn't exist.
        fs::create_dir_all(PathBuf::from("bundled"))?;
        let dst = PathBuf::from(format!("./bundled/{}.rs", self.id));

        // Create a Bundler instance and run it.
        let mut bundler = Bundler::new(src, dst);
        bundler
            .run()
            .context(format!("failed to bundle problem {}", self.id))?;

        Ok(())
    }
}

const MAIN_MOD: &str = "algorist";

lazy_static! {
    static ref RE_MOD: Regex = regex_line(r" (pub  )?mod  (?P<m>.+) ; ").unwrap();
    static ref RE_COMMENT: Regex = regex_line(r" ").unwrap();
    static ref RE_WARN: Regex = regex_line(r" #!\[warn\(.*").unwrap();
    static ref RE_CFG_TEST: Regex = regex_line(r" #\[cfg\(test\)\] ").unwrap();
    static ref RE_ALLOW_DEAD_CODE: Regex = regex_line(r" #.?\[allow\(dead_code\)\] ").unwrap();
    static ref RE_USE: Regex =
        regex_line(format!(" use  {}::(?P<submod>.*)::.*$", MAIN_MOD)).unwrap();
    static ref RE_USE_CRATE: Regex =
        regex_line(r" use  (?P<prefix>\{?)crate::(?P<submod>.*)::(?P<postfix>.*)$").unwrap();
    static ref RE_MACRO_IMPL_CRATE: Regex = regex_line(r" impl \$crate::(?P<content>.*) ").unwrap();
}

struct Bundler {
    src: PathBuf,
    dst: PathBuf,
    out: BufWriter<File>,
    allow: Vec<String>,
}

impl Bundler {
    fn new(src: PathBuf, dst: PathBuf) -> Self {
        Self {
            src,
            dst: dst.clone(),
            out: BufWriter::new(File::create(&dst).unwrap()),
            allow: Vec::new(),
        }
    }

    fn run(&mut self) -> Result<()> {
        let src = self.src.display().to_string();
        let dst = self.dst.display().to_string();
        println!("Bundling {} -> {}", src, dst);

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
            if self.is_ignorable(&line) {
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

        println!("allow: {}", module);

        let reader = mod_reader(module.to_string().replace("::", "/").as_str())?;
        let mut cfg_test_occurred = false;
        let submodules: Vec<String> = reader
            .lines()
            .filter_map(|l| l.ok())
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

        submodules.iter().for_each(|m| {
            self.extend_allow(m).unwrap();
        });

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
        self.writeln(&format!("mod {} {{", MAIN_MOD))?;

        let mut reader = BufReader::new(File::open(&librs)?);
        let mut line = String::new();
        while reader.read_line(&mut line)? > 0 {
            line.pop();
            if self.is_ignorable(&line) {
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
            println!("ignored module: {} (path: {})", mod_name, mod_path);
            return Ok(());
        }

        println!(
            "module: {} (path: {})",
            mod_name,
            mod_path.replace('/', "::")
        );

        self.writeln(&format!("pub mod {} {{", mod_name))?;

        let mut reader = mod_reader(mod_path)?;
        let mut line = String::new();
        while reader.read_line(&mut line)? > 0 {
            line.pop();
            if self.is_ignorable(&line) {
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
        writeln!(self.out, "{}", line).map_err(|e| anyhow!(e))
    }

    fn is_ignorable(&self, line: &str) -> bool {
        RE_COMMENT.is_match(line) || RE_WARN.is_match(line) || RE_ALLOW_DEAD_CODE.is_match(line)
    }
}

fn mod_reader(mod_path: &str) -> Result<BufReader<File>, anyhow::Error> {
    let reader = [
        format!("src/{}.rs", mod_path),
        format!("src/{}/mod.rs", mod_path),
    ]
    .iter()
    .map(|p| File::open(p))
    .find(|r| r.is_ok())
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
