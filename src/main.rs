use clap::{AppSettings, Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
struct Cli {
    /// Path to a configuration file to use
    #[clap(long, short, parse(from_os_str))]
    config: Option<PathBuf>,
    #[clap(long, short)]
    /// Name of the Kimai account to use from the configuration file
    account: Option<String>,
    #[clap(subcommand)]
    commands: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List all possible accounts defined in the configuration file
    Accounts(Accounts),
}

#[derive(Debug, Args)]
#[clap(author)]
struct Accounts {
    /// Path to a configuration file to use
    #[clap(long, short, parse(from_os_str))]
    config: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();
    profitreport::print_profit_report(args.config, args.account)
        .unwrap_or_else(|e| eprintln!("{}", e));
}

#[cfg(test)]
mod main_tests {
    use super::*;
    use clap::IntoApp;

    #[test]
    fn verify_app() {
        Cli::into_app().debug_assert();
    }
}
