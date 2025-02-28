use clap::Subcommand;

pub mod init;
pub mod workspace;

#[derive(Subcommand)]
pub enum Commands {
    /// 初始化 | init
    Init {
        #[command(subcommand)] // 二级子命令
        action: Option<init::Commands>,

        /// 项目名称 | project name
        #[arg(short, long, required = false)]
        name: Option<String>,

        /// 作者 | author
        #[arg(short, long, required = false)]
        author: Option<String>,

        /// 项目版本 | project version
        #[arg(short, long, required = false)]
        version: Option<String>,

        /// 是否初始化git仓库 | init git repository
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        git: bool,

        /// 是否使用默认模板 | use default template
        #[arg(short, long, required = false)]
        default: Option<bool>,

        /// 是否强制覆盖 | force override
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        force: bool,
    },
    /// 工作区 | workspace
    Workspace {
        /// 子命令
        #[command(subcommand)]
        action: Option<workspace::Commands>,
    },
}
