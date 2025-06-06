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

static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/crates/template");

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
        for entry in TEMPLATE_DIR.find(glob).unwrap() {
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

    // For testing purposes, template directory may contain `target` and
    // `Cargo.lock` files. They are ignored by the glob patterns.
    copy("src/**/*", target)?;
    copy("Cargo.toml", target)?;

    Ok(())
}
