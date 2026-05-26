use clap::{Parser, Subcommand};

pub mod cmds;

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
    /// 时间工具, 示例: weap time, weap time -t Asia/Shanghai, weap time -c 60
    Time(cmds::TimeCmd),
    /// 哈希计算, 示例: weap hash "hello"、weap hash -a md5 "hello"、weap hash -f file.txt
    Hash(cmds::HashCmd),
    /// Base64编解码, 示例: weap b64 encode "hello"、weap b64 decode "aGVsbG8="
    B64(cmds::B64Cmd),
    /// 进制转换, 示例: weap base 255、weap base -f 2 "11111111"
    Base(cmds::BaseCmd),
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Uuid(cmd) => cmd.execute(),
        Commands::Pw(cmd) => cmd.execute(),
        Commands::Ip(cmd) => cmd.execute(),
        Commands::Sys(cmd) => cmd.execute(),
        Commands::Time(cmd) => cmd.execute(),
        Commands::Hash(cmd) => cmd.execute(),
        Commands::B64(cmd) => cmd.execute(),
        Commands::Base(cmd) => cmd.execute(),
    }
}
