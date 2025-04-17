use clap::Args;
use rand::{Rng, thread_rng};

#[derive(Args)]
pub struct PwCmd {
    /// 指定密码长度，默认为16位
    #[arg(short = 'l', long, help = "密码长度", default_value = "16")]
    pub length: usize,
}

impl PwCmd {
    /// 执行密码生成命令
    pub fn execute(&self) {
        println!("{}", self.generate());
    }

    /// 生成指定长度的随机密码
    /// 包含大小写字母、数字
    fn generate(&self) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789";
        let mut rng = thread_rng();
        let password: String = (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        password
    }
}
