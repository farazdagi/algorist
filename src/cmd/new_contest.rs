use {
    crate::cmd::{GITIGNORE, RUSTFMT_TOML, SRC_DIR, SubCmd, TPL_DIR, copy, copy_to},
    anyhow::{Context, Result, anyhow},
    argh::FromArgs,
    std::{
        fs,
        path::{Path, PathBuf},
    },
};

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
        copy(&SRC_DIR, "algorist/**/*", &target.join("src"))?;
        copy_to(&TPL_DIR, "lib.rs", &target.join("src/lib.rs"))?;
        copy_to(&TPL_DIR, "Cargo.toml.tpl", &target.join("Cargo.toml"))?;

        // Copy files from root directory.
        fs::write(target.join(".gitignore"), GITIGNORE)?;
        fs::write(target.join("rustfmt.toml"), RUSTFMT_TOML)?;

        // Create files for problems a-h.
        if !self.empty {
            println!("Adding problems a-h to the contest...");
            for letter in 'a'..='h' {
                copy_to(
                    &SRC_DIR,
                    "bin/problem.rs",
                    &target.join(format!("src/bin/{letter}.rs")),
                )?;
            }
        }

        Ok(())
    }
}
