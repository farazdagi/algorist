use argh::FromArgs;

/// The algorist CLI tool.
#[derive(FromArgs, PartialEq, Debug)]
struct TopLevelCmd {
    #[argh(subcommand)]
    nested: TopLevelCmdEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum TopLevelCmdEnum {
    New(NewSubCmd),
}

/// Create new contest.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "new")]
struct NewSubCmd {
    #[argh(option)]
    /// how many x
    x: usize,
}

fn main() {
    let _cmd: TopLevelCmd = if std::env::args()
        .skip(1)
        .next()
        .map_or(false, |s| s.ends_with("algorist"))
    {
        argh::cargo_from_env()
    } else {
        argh::from_env()
    };
}
