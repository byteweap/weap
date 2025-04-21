use clap::{Parser, Subcommand};

mod cmds;

/// 命令行工具主结构
#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = env!("CARGO_PKG_AUTHORS"))]
#[command(about = format!("{}\n\nAuthor: {}", env!("CARGO_PKG_DESCRIPTION"), env!("CARGO_PKG_AUTHORS")))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// 支持的命令列表
#[derive(Subcommand)]
enum Commands {
    /// UUID生成器, 示例: weap uuid、weap uuid -x
    Uuid(cmds::UuidCmd),
    /// 密码生成器, 示例: weap pw
    Pw(cmds::PwCmd),
    /// IP查询, 示例: weap ip
    Ip(cmds::IpCmd),
    /// 系统信息查询, 示例: weap sys
    Sys(cmds::SysCmd),
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Uuid(cmd) => cmd.execute(),
        Commands::Pw(cmd) => cmd.execute(),
        Commands::Ip(cmd) => cmd.execute(),
        Commands::Sys(cmd) => cmd.execute(),
    }
}
