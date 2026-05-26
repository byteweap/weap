use clap::{Args, ValueEnum};
use sha2::{Digest, Sha256, Sha512};
use std::io::Read;

#[derive(ValueEnum, Clone, Debug)]
pub enum HashAlgo {
    Md5,
    Sha256,
    Sha512,
}

#[derive(Args)]
pub struct HashCmd {
    /// 输入文本（与 -f 二选一）
    pub text: Option<String>,

    /// 指定算法
    #[arg(short, long, value_enum, default_value = "sha256")]
    pub algo: HashAlgo,

    /// 从文件读取（与文本输入二选一）
    #[arg(short, long)]
    pub file: Option<String>,
}

impl HashCmd {
    pub fn execute(&self) {
        match self.compute() {
            Ok(hash) => println!("{}", hash),
            Err(e) => eprintln!("错误: {}", e),
        }
    }

    pub fn compute(&self) -> Result<String, String> {
        let data = self.read_input()?;
        let hash = match self.algo {
            HashAlgo::Md5 => format!("{:x}", md5::compute(&data)),
            HashAlgo::Sha256 => format!("{:x}", Sha256::digest(&data)),
            HashAlgo::Sha512 => format!("{:x}", Sha512::digest(&data)),
        };
        Ok(hash)
    }

    fn read_input(&self) -> Result<Vec<u8>, String> {
        if let Some(path) = &self.file {
            let mut file = std::fs::File::open(path)
                .map_err(|e| format!("无法打开文件 '{}': {}", path, e))?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)
                .map_err(|e| format!("读取文件失败: {}", e))?;
            Ok(buf)
        } else if let Some(text) = &self.text {
            Ok(text.as_bytes().to_vec())
        } else {
            let mut buf = Vec::new();
            std::io::stdin()
                .read_to_end(&mut buf)
                .map_err(|e| format!("读取标准输入失败: {}", e))?;
            Ok(buf)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_text() {
        let cmd = HashCmd {
            text: Some("hello".to_string()),
            algo: HashAlgo::Md5,
            file: None,
        };
        assert_eq!(cmd.compute().unwrap(), "5d41402abc4b2a76b9719d911017c592");
    }

    #[test]
    fn test_sha256_text() {
        let cmd = HashCmd {
            text: Some("hello".to_string()),
            algo: HashAlgo::Sha256,
            file: None,
        };
        assert_eq!(
            cmd.compute().unwrap(),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_sha512_text() {
        let cmd = HashCmd {
            text: Some("hello".to_string()),
            algo: HashAlgo::Sha512,
            file: None,
        };
        assert_eq!(
            cmd.compute().unwrap(),
            "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"
        );
    }

    #[test]
    fn test_md5_empty() {
        let cmd = HashCmd {
            text: Some("".to_string()),
            algo: HashAlgo::Md5,
            file: None,
        };
        assert_eq!(cmd.compute().unwrap(), "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn test_sha256_empty() {
        let cmd = HashCmd {
            text: Some("".to_string()),
            algo: HashAlgo::Sha256,
            file: None,
        };
        assert_eq!(
            cmd.compute().unwrap(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_file_hash() {
        let path = "/tmp/weap_test_hash.txt";
        std::fs::write(path, "hello").unwrap();
        let cmd = HashCmd {
            text: None,
            algo: HashAlgo::Md5,
            file: Some(path.to_string()),
        };
        assert_eq!(cmd.compute().unwrap(), "5d41402abc4b2a76b9719d911017c592");
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_file_not_found() {
        let cmd = HashCmd {
            text: None,
            algo: HashAlgo::Md5,
            file: Some("/tmp/nonexistent_file_weap.txt".to_string()),
        };
        assert!(cmd.compute().is_err());
    }

    #[test]
    fn test_execute_md5() {
        let cmd = HashCmd {
            text: Some("test".to_string()),
            algo: HashAlgo::Md5,
            file: None,
        };
        cmd.execute();
    }

    #[test]
    fn test_execute_sha256() {
        let cmd = HashCmd {
            text: Some("test".to_string()),
            algo: HashAlgo::Sha256,
            file: None,
        };
        cmd.execute();
    }

    #[test]
    fn test_execute_sha512() {
        let cmd = HashCmd {
            text: Some("test".to_string()),
            algo: HashAlgo::Sha512,
            file: None,
        };
        cmd.execute();
    }

    #[test]
    fn test_execute_file_error() {
        let cmd = HashCmd {
            text: None,
            algo: HashAlgo::Md5,
            file: Some("/tmp/nonexistent_weap.txt".to_string()),
        };
        cmd.execute();
    }
}
