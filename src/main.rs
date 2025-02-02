use anyhow::Result;
use clap::Parser;
use minigrep::{run, Args};

fn main() -> Result<()> {
    let args = Args::parse(); // 解析命令行参数
    run(args)?;

    Ok(())
}
