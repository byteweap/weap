use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use clap::{Args, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum B64Action {
    Encode,
    Decode,
}

#[derive(Args)]
pub struct B64Cmd {
    /// 操作类型
    #[arg(value_enum)]
    pub action: B64Action,

    /// 输入文本
    pub text: String,
}

impl B64Cmd {
    pub fn execute(&self) {
        match self.process() {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("错误: {}", e),
        }
    }

    pub fn process(&self) -> Result<String, String> {
        match self.action {
            B64Action::Encode => Ok(STANDARD.encode(self.text.as_bytes())),
            B64Action::Decode => {
                let bytes = STANDARD
                    .decode(&self.text)
                    .map_err(|e| format!("无效的 Base64 输入: {}", e))?;
                String::from_utf8(bytes)
                    .map_err(|e| format!("解码结果不是有效的 UTF-8: {}", e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_hello() {
        let cmd = B64Cmd {
            action: B64Action::Encode,
            text: "hello".to_string(),
        };
        assert_eq!(cmd.process().unwrap(), "aGVsbG8=");
    }

    #[test]
    fn test_decode_hello() {
        let cmd = B64Cmd {
            action: B64Action::Decode,
            text: "aGVsbG8=".to_string(),
        };
        assert_eq!(cmd.process().unwrap(), "hello");
    }

    #[test]
    fn test_encode_empty() {
        let cmd = B64Cmd {
            action: B64Action::Encode,
            text: "".to_string(),
        };
        assert_eq!(cmd.process().unwrap(), "");
    }

    #[test]
    fn test_decode_empty() {
        let cmd = B64Cmd {
            action: B64Action::Decode,
            text: "".to_string(),
        };
        assert_eq!(cmd.process().unwrap(), "");
    }

    #[test]
    fn test_encode_chinese() {
        let cmd = B64Cmd {
            action: B64Action::Encode,
            text: "你好".to_string(),
        };
        let encoded = cmd.process().unwrap();
        let decode_cmd = B64Cmd {
            action: B64Action::Decode,
            text: encoded,
        };
        assert_eq!(decode_cmd.process().unwrap(), "你好");
    }

    #[test]
    fn test_encode_special_chars() {
        let cmd = B64Cmd {
            action: B64Action::Encode,
            text: "Hello, World! @#$%^&*()".to_string(),
        };
        let encoded = cmd.process().unwrap();
        let decode_cmd = B64Cmd {
            action: B64Action::Decode,
            text: encoded,
        };
        assert_eq!(decode_cmd.process().unwrap(), "Hello, World! @#$%^&*()");
    }

    #[test]
    fn test_decode_invalid() {
        let cmd = B64Cmd {
            action: B64Action::Decode,
            text: "!!!invalid!!!".to_string(),
        };
        assert!(cmd.process().is_err());
    }

    #[test]
    fn test_roundtrip_long_text() {
        let original = "The quick brown fox jumps over the lazy dog. 0123456789";
        let cmd = B64Cmd {
            action: B64Action::Encode,
            text: original.to_string(),
        };
        let encoded = cmd.process().unwrap();
        let decode_cmd = B64Cmd {
            action: B64Action::Decode,
            text: encoded,
        };
        assert_eq!(decode_cmd.process().unwrap(), original);
    }

    #[test]
    fn test_execute_encode() {
        let cmd = B64Cmd {
            action: B64Action::Encode,
            text: "test".to_string(),
        };
        cmd.execute();
    }

    #[test]
    fn test_execute_decode() {
        let cmd = B64Cmd {
            action: B64Action::Decode,
            text: "dGVzdA==".to_string(),
        };
        cmd.execute();
    }

    #[test]
    fn test_execute_decode_error() {
        let cmd = B64Cmd {
            action: B64Action::Decode,
            text: "!!!".to_string(),
        };
        cmd.execute();
    }
}
