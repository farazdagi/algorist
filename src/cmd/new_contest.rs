use {
    crate::cmd::SubCmd,
    anyhow::{Context, Result, anyhow},
    argh::FromArgs,
    include_dir::{Dir, include_dir},
    std::{
        fs,
        path::{Path, PathBuf},
    },
};

static SRC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src");
pub static TPL_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/tpl");
static RUSTFMT_TOML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/rustfmt.toml"));

/// Create a new contest project.
#[derive(FromArgs)]
#[argh(subcommand, name = "new")]
pub struct NewContestSubCmd {
    #[argh(positional)]
    /// contest ID
    id: String,

    #[argh(switch)]
    /// no problems will be added to the contest, use `add` command to add
    /// problems later
    empty: bool,
}

impl SubCmd for NewContestSubCmd {
    fn run(&self) -> Result<()> {
        let root_dir = PathBuf::from("./")
            .canonicalize()
            .context("failed to canonicalize root directory path")?
            .join(&self.id);

        // Ensure that the root directory does not already exist.
        // Create "src" directory for the contest (if it doesn't exist).
        let src_dir = root_dir.join("src");
        if root_dir.exists() || src_dir.exists() {
            return Err(anyhow!("Directory already exists: {:?}", root_dir));
        }
        fs::create_dir_all(src_dir)?;

        // Copy template files into the contest directory.
        self.copy_template(&root_dir)
            .context("failed to copy template files")?;

        println!("New contest created at {root_dir:?}");
        Ok(())
    }
}

impl NewContestSubCmd {
    fn copy_template(&self, target: &Path) -> std::io::Result<()> {
        // Copy the necessary library files for contest project.
        println!("Copying template files to the contest directory...");
        copy(&SRC_DIR, "lib.rs", &target.join("src"))?;
        copy(&SRC_DIR, "io/**/*", &target.join("src"))?;
        copy(&SRC_DIR, "collections/**/*", &target.join("src"))?;
        copy(&SRC_DIR, "ext/**/*", &target.join("src"))?;
        copy(&SRC_DIR, "math/**/*", &target.join("src"))?;
        copy(&SRC_DIR, "misc/**/*", &target.join("src"))?;
        copy_to(&TPL_DIR, "Cargo.toml.tpl", &target.join("Cargo.toml"))?;
        copy_to(&TPL_DIR, "README.md", &target.join("README.md"))?;

        // Copy files from root directory.
        fs::write(target.join("rustfmt.toml"), RUSTFMT_TOML)?;

        // Create files for problems a-h.
        if !self.empty {
            println!("Adding problems a-h to the contest...");
            for letter in 'a'..='h' {
                copy_to(
                    &TPL_DIR,
                    "problem.rs",
                    &target.join(format!("src/bin/{letter}.rs")),
                )?;
            }
        }

        Ok(())
    }
}

pub fn copy(dir: &Dir, glob: &str, target: &Path) -> std::io::Result<()> {
    for entry in dir.find(glob).unwrap() {
        if let Some(file) = entry.as_file() {
            let rel_path = file.path();
            let dest_path = target.join(rel_path);
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(dest_path, file.contents())?;
        }
    }
    Ok(())
}

pub fn copy_to(dir: &Dir, src: &str, target: &Path) -> std::io::Result<()> {
    let file = dir
        .get_file(src)
        .unwrap_or_else(|| panic!("file should exist in template directory: {src}"));
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(target, file.contents())
}
