use clap::Args;
use uuid::Uuid;

/// UUID生成器
#[derive(Args)]
pub struct UuidCmd {
    /// 是否生成不带连字符的UUID
    #[arg(
        short = 'x',
        long,
        help = "生成不带连字符的UUID",
        default_value = "false"
    )]
    pub no_hyphens: bool,
}

impl UuidCmd {
    /// 执行UUID生成命令
    pub fn execute(&self) {
        println!("{}", self.generate());
    }

    /// 生成UUID
    /// 根据no_hyphens参数决定是否包含连字符
    fn generate(&self) -> String {
        let uuid = Uuid::new_v4();
        if self.no_hyphens {
            uuid.to_string().replace("-", "")
        } else {
            uuid.to_string()
        }
    }
}
