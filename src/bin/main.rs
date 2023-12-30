use erm_tools::core;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version = "1.0", author = "luokai")]
struct Opts {
    #[clap(short, long, default_value = "erm-tools.json")]
    config: String,
    // #[clap(subcommand)]
    // subcmd: SubCommand,
}

fn main() {
    let opts: Opts = Opts::parse();
    match core::load_env(&opts.config) {
        Ok(()) => {
            core::exec(&mut core::env());
        }
        Err(e) => {
            eprint!("{:?}", e);
            panic!("初始化配置失败!!!");
        }
    }
}
