use {
    anyhow::{Context, Result, anyhow},
    argh::FromArgs,
    std::{
        fs::{copy, create_dir_all},
        path::PathBuf,
    },
};

/// The algorist CLI tool.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(help_triggers("-h", "--help", "help"))]
struct TopLevelCmd {
    #[argh(subcommand)]
    nested: TopLevelCmdEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum TopLevelCmdEnum {
    New(NewSubCmd),
}

/// Create a new contest project with the given contest ID.
/// It creates a directory structure and necessary Rust files for the contest.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "new")]
struct NewSubCmd {
    #[argh(positional)]
    /// contest ID
    id: String,
}

fn main() -> Result<()> {
    // Allow the CLI to be run as `cargo algorist` or `algorist`.
    let cmd: TopLevelCmd = if std::env::args()
        .skip(1)
        .next()
        .map_or(false, |s| s.ends_with("algorist"))
    {
        argh::cargo_from_env()
    } else {
        argh::from_env()
    };

    match cmd.nested {
        TopLevelCmdEnum::New(new_cmd) => {
            let root_dir = PathBuf::from("./")
                .canonicalize()
                .context("failed to canonicalize root directory path")?
                .join(format!("contest-{}", new_cmd.id));

            // Create "src" directory for the contest (if it doesn't exist).
            let src_dir = root_dir.join("src");
            if src_dir.exists() {
                return Err(anyhow!("Directory already exists: {:?}", root_dir));
            }
            create_dir_all(src_dir)?;

            // Copy the template files into the src directory.
            let template_dir = PathBuf::from("crates/template")
                .canonicalize()
                .context("template directory not found")?;

            let files_to_copy = ["Cargo.toml", "src/main.rs"];
            for file in files_to_copy {
                let src_path = template_dir.join(file);
                if !src_path.exists() {
                    return Err(anyhow!("Template file not found: {:?}", src_path));
                }
                let dest_path = root_dir.join(file);
                copy(&src_path, &dest_path)
                    .context(format!("failed to copy {:?} to {:?}", src_path, dest_path))?;
            }

            println!("New contest created at {:?}", root_dir);
        }
    }

    Ok(())
}
