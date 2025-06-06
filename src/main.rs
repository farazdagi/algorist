mod cmd;

use {
    crate::cmd::SubCmd,
    anyhow::{Context, Result},
    argh::FromArgs,
    cmd::new_contest::NewSubCmd,
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

impl TopLevelCmd {
    /// Run the nested command.
    fn run(&self) -> Result<()> {
        match &self.nested {
            TopLevelCmdEnum::New(new_cmd) => new_cmd.run(),
        }
    }
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

    cmd.run().context("failed to run subcommand")
}
