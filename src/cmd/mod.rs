pub mod bundle;
pub mod new_contest;

use {anyhow::Result, argh::FromArgs, bundle::BundleSubCmd, new_contest::NewSubCmd};

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
    New(NewSubCmd),
    Bundle(BundleSubCmd),
}

impl MainCmd {
    /// Run the nested command.
    pub fn run(&self) -> Result<()> {
        match &self.nested {
            TopLevelCmdEnum::New(new_cmd) => new_cmd.run(),
            TopLevelCmdEnum::Bundle(bundle_cmd) => bundle_cmd.run(),
        }
    }
}
