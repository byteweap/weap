use clap::Args;

#[derive(Args)]
pub struct BaseCmd {
    /// 要转换的数字（十进制）
    pub value: String,

    /// 输入的进制（2-36），默认为 10
    #[arg(short, long, default_value = "10")]
    pub from: u32,
}

impl BaseCmd {
    pub fn execute(&self) {
        match self.convert() {
            Ok(results) => {
                for line in results {
                    println!("{}", line);
                }
            }
            Err(e) => eprintln!("错误: {}", e),
        }
    }

    pub fn convert(&self) -> Result<Vec<String>, String> {
        let value = self.value.trim();
        if self.from < 2 || self.from > 36 {
            return Err(format!("进制必须在 2-36 之间，当前为 {}", self.from));
        }
        let n = u64::from_str_radix(value, self.from)
            .map_err(|_| format!("'{}' 不是有效的 {} 进制数", value, self.from))?;

        Ok(vec![
            format!("二进制 (bin):  {}", self.to_base(n, 2)),
            format!("八进制 (oct):  {}", self.to_base(n, 8)),
            format!("十进制 (dec):  {}", n),
            format!("十六进制 (hex): {}", self.to_base(n, 16)),
        ])
    }

    fn to_base(&self, mut n: u64, base: u64) -> String {
        if n == 0 {
            return "0".to_string();
        }
        let chars: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyz".chars().collect();
        let mut result = Vec::new();
        while n > 0 {
            result.push(chars[(n % base) as usize]);
            n /= base;
        }
        result.reverse();
        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_255() {
        let cmd = BaseCmd {
            value: "255".to_string(),
            from: 10,
        };
        let result = cmd.convert().unwrap();
        assert!(result[0].contains("11111111"));
        assert!(result[1].contains("377"));
        assert!(result[2].contains("255"));
        assert!(result[3].contains("ff"));
    }

    #[test]
    fn test_decimal_0() {
        let cmd = BaseCmd {
            value: "0".to_string(),
            from: 10,
        };
        let result = cmd.convert().unwrap();
        assert!(result[0].contains("0"));
        assert!(result[1].contains("0"));
        assert!(result[2].contains("0"));
        assert!(result[3].contains("0"));
    }

    #[test]
    fn test_decimal_1() {
        let cmd = BaseCmd {
            value: "1".to_string(),
            from: 10,
        };
        let result = cmd.convert().unwrap();
        assert!(result[0].contains("1"));
        assert!(result[3].contains("1"));
    }

    #[test]
    fn test_from_binary() {
        let cmd = BaseCmd {
            value: "11111111".to_string(),
            from: 2,
        };
        let result = cmd.convert().unwrap();
        assert!(result[2].contains("255"));
        assert!(result[3].contains("ff"));
    }

    #[test]
    fn test_from_hex() {
        let cmd = BaseCmd {
            value: "ff".to_string(),
            from: 16,
        };
        let result = cmd.convert().unwrap();
        assert!(result[0].contains("11111111"));
        assert!(result[2].contains("255"));
    }

    #[test]
    fn test_from_octal() {
        let cmd = BaseCmd {
            value: "377".to_string(),
            from: 8,
        };
        let result = cmd.convert().unwrap();
        assert!(result[2].contains("255"));
        assert!(result[3].contains("ff"));
    }

    #[test]
    fn test_large_number() {
        let cmd = BaseCmd {
            value: "4294967295".to_string(),
            from: 10,
        };
        let result = cmd.convert().unwrap();
        assert!(result[3].contains("ffffffff"));
    }

    #[test]
    fn test_invalid_input() {
        let cmd = BaseCmd {
            value: "xyz".to_string(),
            from: 10,
        };
        assert!(cmd.convert().is_err());
    }

    #[test]
    fn test_invalid_base() {
        let cmd = BaseCmd {
            value: "123".to_string(),
            from: 37,
        };
        // u64::from_str_radix with base > 36 will fail
        assert!(cmd.convert().is_err());
    }

    #[test]
    fn test_hex_with_prefix() {
        let cmd = BaseCmd {
            value: "0xff".to_string(),
            from: 16,
        };
        // "0xff" is not valid hex (contains 'x')
        assert!(cmd.convert().is_err());
    }

    #[test]
    fn test_execute_decimal() {
        let cmd = BaseCmd {
            value: "255".to_string(),
            from: 10,
        };
        cmd.execute();
    }

    #[test]
    fn test_execute_binary() {
        let cmd = BaseCmd {
            value: "11111111".to_string(),
            from: 2,
        };
        cmd.execute();
    }

    #[test]
    fn test_execute_error() {
        let cmd = BaseCmd {
            value: "xyz".to_string(),
            from: 10,
        };
        cmd.execute();
    }
}
