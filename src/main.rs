use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short, parse(from_os_str))]
    config: Option<PathBuf>,
    #[clap(long, short)]
    account: Option<String>,
}

fn main() {
    let args = Args::parse();
    profitreport::print_profit_report(args.config, args.account)
        .unwrap_or_else(|e| eprintln!("{}", e));
}

#[cfg(test)]
mod main_tests {
    use super::*;
    use clap::IntoApp;

    #[test]
    fn verify_app() {
        Args::into_app().debug_assert();
    }
}
