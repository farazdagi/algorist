pub mod add_problem;
pub mod bundle_problem;
pub mod new_contest;

use add_problem::AddProblemSubCmd;
use {
    anyhow::Result,
    argh::FromArgs,
    bundle_problem::BundleProblemSubCmd,
    new_contest::NewContestSubCmd,
};

pub trait SubCmd {
    fn run(&self) -> anyhow::Result<()>;
}

/// The algorist CLI tool.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(help_triggers("-h", "--help", "help"))]
pub struct MainCmd {
    #[argh(subcommand)]
    nested: TopLevelCmdEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum TopLevelCmdEnum {
    New(NewContestSubCmd),
    Bundle(BundleProblemSubCmd),
    Add(AddProblemSubCmd),
}

impl MainCmd {
    /// Run the nested command.
    pub fn run(&self) -> Result<()> {
        match &self.nested {
            TopLevelCmdEnum::New(new_cmd) => new_cmd.run(),
            TopLevelCmdEnum::Bundle(bundle_cmd) => bundle_cmd.run(),
            TopLevelCmdEnum::Add(add_cmd) => add_cmd.run(),
        }
    }
}
