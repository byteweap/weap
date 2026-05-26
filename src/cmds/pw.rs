use clap::Args;
use rand::{Rng, thread_rng};

#[derive(Args)]
pub struct PwCmd {
    /// 指定密码长度，默认为16位
    #[arg(short = 'l', long, help = "密码长度", default_value = "16")]
    pub length: usize,

    /// 是否包含特殊字符
    #[arg(short = 's', long, help = "包含特殊字符", default_value = "false")]
    pub special: bool,
}

impl PwCmd {
    /// 执行密码生成命令
    pub fn execute(&self) {
        println!("{}", self.generate());
    }

    /// 生成指定长度的随机密码
    /// 包含大小写字母、数字
    pub fn generate(&self) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789";
        const SPECIAL_CHARSET: &[u8] = b"!@#$%^&*()-_=+[]{}|;:',.<>?/`~";
        let mut rng = thread_rng();
        let charset = if self.special {
            [CHARSET, SPECIAL_CHARSET].concat()
        } else {
            CHARSET.to_vec()
        };
        let password: String = (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .collect();
        password
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_default_length() {
        let cmd = PwCmd {
            length: 16,
            special: false,
        };
        let pw = cmd.generate();
        assert_eq!(pw.len(), 16);
        assert!(pw.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_generate_custom_length() {
        let cmd = PwCmd {
            length: 32,
            special: false,
        };
        let pw = cmd.generate();
        assert_eq!(pw.len(), 32);
    }

    #[test]
    fn test_generate_length_1() {
        let cmd = PwCmd {
            length: 1,
            special: false,
        };
        let pw = cmd.generate();
        assert_eq!(pw.len(), 1);
    }

    #[test]
    fn test_generate_length_0() {
        let cmd = PwCmd {
            length: 0,
            special: false,
        };
        let pw = cmd.generate();
        assert_eq!(pw.len(), 0);
        assert!(pw.is_empty());
    }

    #[test]
    fn test_generate_with_special_chars() {
        let cmd = PwCmd {
            length: 100,
            special: true,
        };
        let pw = cmd.generate();
        assert_eq!(pw.len(), 100);
        // 大样本下应包含特殊字符
        let has_special = pw.chars().any(|c| !c.is_ascii_alphanumeric());
        assert!(has_special, "密码应包含特殊字符");
    }

    #[test]
    fn test_generate_without_special_chars() {
        let cmd = PwCmd {
            length: 100,
            special: false,
        };
        let pw = cmd.generate();
        assert_eq!(pw.len(), 100);
        assert!(pw.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_generate_uniqueness() {
        let cmd = PwCmd {
            length: 32,
            special: false,
        };
        let pw1 = cmd.generate();
        let pw2 = cmd.generate();
        assert_ne!(pw1, pw2);
    }

    #[test]
    fn test_execute_default() {
        let cmd = PwCmd {
            length: 16,
            special: false,
        };
        cmd.execute();
    }

    #[test]
    fn test_execute_with_special() {
        let cmd = PwCmd {
            length: 20,
            special: true,
        };
        cmd.execute();
    }
}
