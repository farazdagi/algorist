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
static TPL_CARGO_TOML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml.tpl"));
static RUSTFMT_TOML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/rustfmt.toml"));
static README_MD: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"));

/// Create a new contest project with the given contest ID.
/// It creates a directory structure and necessary Rust files for the contest.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "new")]
pub struct NewSubCmd {
    #[argh(positional)]
    /// contest ID
    id: String,
}

impl SubCmd for NewSubCmd {
    fn run(&self) -> Result<()> {
        let root_dir = PathBuf::from("./")
            .canonicalize()
            .context("failed to canonicalize root directory path")?
            .join(format!("contest-{}", self.id));

        // Create "src" directory for the contest (if it doesn't exist).
        let src_dir = root_dir.join("src");
        if src_dir.exists() {
            return Err(anyhow!("Directory already exists: {:?}", root_dir));
        }
        fs::create_dir_all(src_dir)?;

        // Copy template files into the contest directory.
        copy_template(&root_dir).context("failed to copy template files")?;

        println!("New contest created at {:?}", root_dir);
        Ok(())
    }
}

fn copy_template(target: &Path) -> std::io::Result<()> {
    fn copy(glob: &str, target: &Path) -> std::io::Result<()> {
        for entry in SRC_DIR.find(glob).unwrap() {
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

    fn copy_to(src: &str, target: &Path) -> std::io::Result<()> {
        let file = SRC_DIR
            .get_file(src)
            .expect(format!("file should exist in template directory: {}", src).as_str());
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(target, file.contents())
    }

    // Copy the necessary library files for contest project.
    copy("lib.rs", &target.join("src"))?;
    copy("io/**/*", &target.join("src"))?;

    // Copy files from root directory.
    fs::write(target.join("rustfmt.toml"), RUSTFMT_TOML)?;
    fs::write(target.join("Cargo.toml"), TPL_CARGO_TOML)?;
    fs::write(target.join("README.md"), README_MD)?;

    // Make copies of `src/bin/a.rs` file.
    // This is to create files for problems a-h.
    for letter in 'a'..='h' {
        copy_to("bin/a.rs", &target.join(format!("src/bin/{}.rs", letter)))?;
    }

    Ok(())
}
