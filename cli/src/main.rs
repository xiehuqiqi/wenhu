use clap::{CommandFactory, Parser};
use colored::*;

mod commands;
use commands::Commands;

use commands::init;
use commands::init::Commands as InitCommands;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const CLINAME: &str = "wenhu";
const DESCRIPTION: &str = "✨ 文狐 - 项目调试开发管理工具 | Wenhu Project management tool";
const BUILD_TIME: &str = env!("BUILD_TIMESTAMP");
const LICENSE: &str = env!("CARGO_PKG_LICENSE");
const AUTHOR: &str = "xiehuqiqi";

#[derive(Parser)]
#[command(
    name = CLINAME,
    author = AUTHOR,
    version = VERSION,
    about = DESCRIPTION,
    long_about = None,
    about = DESCRIPTION
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();
    let mut cmd = Cli::command();

    cmd = cmd
        .help_template(format!(
            r#"{{before-help}}{{about-section}}{}
{{usage-heading}} {{usage}}
{{all-args}}

{}{{after-help}}"#,
            "──────────────────────────────────────────────────────────────────────────"
                .bright_cyan(),
            "──────────────────────────────────────────────────────────────────────────"
                .bright_cyan()
        ))
        .before_help(format!(
            r#"> {} ──── @{} "#,
            CLINAME.red().bold(),
            VERSION.bright_cyan()
        ))
        .after_help(format!(
            r#"{}:
  # {}: {} - {}                             
  # {}: @{}
  # {}: {}
  # {}: {}  
{}"#,
            "Packet information".bright_magenta().bold(),
            "Name".bright_cyan(),
            CLINAME.red().bold(),
            VERSION.bright_cyan(),
            "Author".bright_cyan(),
            AUTHOR.yellow(),
            "License".bright_cyan(),
            LICENSE.bright_blue(),
            "Build-Time".bright_cyan(),
            BUILD_TIME.bright_white(),
            "──────────────────────────────────────────────────────────────────────────"
                .bright_cyan()
        ));

    if let Some(Commands::Init {
        action,
        name,
        author,
        version,
        git,
        default,
        force,
    }) = cli.command
    {
        let project_name = name.unwrap_or_default(); // 默认空字符串
        let project_author = author.unwrap_or_else(|| "".to_string()); // 带默认值
        let project_version = version.unwrap_or_else(|| "0.1.0".to_string());
        let is_default = default.unwrap_or(true);

        match action {
            None => init::init(),
            Some(InitCommands::Workspace { name, path }) => init::workspace(),
            Some(InitCommands::Packages { name, path }) => init::packages(),
        }
    } else {
        cmd.print_help().unwrap();
    }
}
