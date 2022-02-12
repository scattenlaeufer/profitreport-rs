use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short, parse(from_os_str))]
    config: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    profitreport::print_profit_report(args.config).unwrap_or_else(|e| eprintln!("{}", e));
}
