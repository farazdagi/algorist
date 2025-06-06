pub mod new_contest;

pub trait SubCmd {
    fn run(&self) -> anyhow::Result<()>;
}
