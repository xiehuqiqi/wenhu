use clap::{CommandFactory, Parser, Subcommand};
// use colored::*;

// 在文件顶部添加版本常量
const VERSION: &str = env!("CARGO_PKG_VERSION");
const CLINAME: &str = "wenhu";
const DESCRIPTION: &str =
    "Wenhu Project debugging development management tool | 文狐 项目调试开发管理工具";

#[derive(Parser)]
#[command(name = CLINAME, version = VERSION, about = DESCRIPTION)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 显示当前版本
    Version,
    /// 初始化新项目
    Init,
    /// 创建新模块
    New,
    /// 启动开发服务器
    Run,
}

fn main() {
    let cli = Cli::parse();
    let mut cmd = Cli::command();

    match cli.command {
        None => {
            cmd.print_help().unwrap();
        }
        Some(Commands::Version) => {
            println!("{} Version: {}", CLINAME, VERSION);
        }
        Some(Commands::Init) => {
            println!("init");
        }
        Some(Commands::New) => {
            println!("new");
        }
        Some(Commands::Run) => {
            println!("run");
        }
    }
}
