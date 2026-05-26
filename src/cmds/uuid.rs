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
    pub fn generate(&self) -> String {
        let uuid = Uuid::new_v4();
        if self.no_hyphens {
            uuid.simple().to_string()
        } else {
            uuid.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_with_hyphens() {
        let cmd = UuidCmd { no_hyphens: false };
        let uuid = cmd.generate();
        assert_eq!(uuid.len(), 36);
        assert_eq!(uuid.chars().filter(|c| *c == '-').count(), 4);
        // 验证格式: 8-4-4-4-12
        let parts: Vec<&str> = uuid.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
        assert_eq!(parts[4].len(), 12);
        // 验证只包含十六进制字符
        assert!(uuid.chars().all(|c| c.is_ascii_hexdigit() || c == '-'));
    }

    #[test]
    fn test_generate_no_hyphens() {
        let cmd = UuidCmd { no_hyphens: true };
        let uuid = cmd.generate();
        assert_eq!(uuid.len(), 32);
        assert!(!uuid.contains('-'));
        assert!(uuid.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_uniqueness() {
        let cmd = UuidCmd { no_hyphens: false };
        let uuid1 = cmd.generate();
        let uuid2 = cmd.generate();
        assert_ne!(uuid1, uuid2);
    }

    #[test]
    fn test_generate_no_hyphens_uniqueness() {
        let cmd = UuidCmd { no_hyphens: true };
        let uuid1 = cmd.generate();
        let uuid2 = cmd.generate();
        assert_ne!(uuid1, uuid2);
    }

    #[test]
    fn test_execute_with_hyphens() {
        let cmd = UuidCmd { no_hyphens: false };
        cmd.execute();
    }

    #[test]
    fn test_execute_no_hyphens() {
        let cmd = UuidCmd { no_hyphens: true };
        cmd.execute();
    }
}
