use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// 工作区 | workspace
    Workspace {
        /// 名称 | name
        #[arg(short, long, required = false)]
        name: Option<String>,
        /// path | 路径
        #[arg(short, long, required = false)]
        path: Option<String>,
    },
    /// 包 | package
    Packages {
        /// 名称 | name
        #[arg(short, long, required = false)]
        name: Option<String>,
        /// path | 路径
        #[arg(short, long, required = false)]
        path: Option<String>,
    },
}

pub fn init() {
    println!(
        "xxxxxxxxxxx init " // "{} {} {} {} {}",
                            // project_name, project_author, project_version, is_workspace, "init"
    );
}

pub fn workspace() {
    println!(
        "work" // "{} {} {} {} {} {} {} {} {}",
               // project_name,
               // project_author,
               // project_version,
               // is_workspace,
               // is_git,
               // is_default,
               // is_force,
               // "init"
    );
}

pub fn packages() {
    println!(
        "packages" // "{} {} {} {} {} {} {} {} {}",
                   // project_name,
                   // project_author,
                   // project_version,
                   // "{} {} {} {} {} {} {} {} {}",
                   // project_name, project_author, project_version, is_workspace, is_git,
    )
}
