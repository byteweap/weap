use chrono::{DateTime, Duration, Local, Utc};
use chrono_tz::Tz;
use clap::Args;
use std::str::FromStr;

/// 时间工具
#[derive(Args)]
pub struct TimeCmd {
    /// 显示指定时区的时间，例如: Asia/Shanghai, America/New_York
    #[arg(short, long)]
    pub timezone: Option<String>,

    /// 显示Unix时间戳
    #[arg(short, long)]
    pub unix: bool,

    /// 格式化时间，例如: %Y-%m-%d %H:%M:%S
    #[arg(short, long, default_value = "%Y-%m-%d %H:%M:%S")]
    pub format: String,

    /// 倒计时秒数
    #[arg(short, long)]
    pub countdown: Option<u64>,
}

impl TimeCmd {
    pub fn execute(&self) {
        if let Some(seconds) = self.countdown {
            self.run_countdown(seconds);
            return;
        }

        self.print_current_time();
    }

    fn print_current_time(&self) {
        let now_local: DateTime<Local> = Local::now();
        let now_utc: DateTime<Utc> = Utc::now();

        println!("当前本地时间: {}", now_local.format(&self.format));
        println!("当前UTC时间: {}", now_utc.format(&self.format));

        if self.unix {
            println!("Unix时间戳: {}", now_utc.timestamp());
        }

        if let Some(tz_str) = &self.timezone {
            match Tz::from_str(tz_str) {
                Ok(tz) => {
                    let time_in_tz = now_utc.with_timezone(&tz);
                    println!("{}时间: {}", tz_str, time_in_tz.format(&self.format));
                }
                Err(_) => {
                    println!("错误: 无效的时区 '{}'", tz_str);
                    println!("提示: 时区格式应为 'Region/City'，例如 'Asia/Shanghai'");
                }
            }
        }
    }

    fn run_countdown(&self, seconds: u64) {
        println!("开始倒计时 {} 秒", seconds);

        let start_time = Local::now();
        let end_time = start_time + Duration::seconds(seconds as i64);

        while Local::now() < end_time {
            let remaining = end_time - Local::now();
            let remaining_secs = remaining.num_seconds();

            print!(
                "\r剩余时间: {:02}:{:02}",
                remaining_secs / 60,
                remaining_secs % 60
            );

            let _ = std::io::Write::flush(&mut std::io::stdout());
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        println!("\n倒计时结束!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_default() {
        let cmd = TimeCmd {
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            unix: false,
            timezone: None,
            countdown: None,
        };
        cmd.execute();
    }

    #[test]
    fn test_execute_with_unix() {
        let cmd = TimeCmd {
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            unix: true,
            timezone: None,
            countdown: None,
        };
        cmd.execute();
    }

    #[test]
    fn test_execute_with_valid_timezone() {
        let cmd = TimeCmd {
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            unix: false,
            timezone: Some("Asia/Shanghai".to_string()),
            countdown: None,
        };
        cmd.execute();
    }

    #[test]
    fn test_execute_with_invalid_timezone() {
        let cmd = TimeCmd {
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            unix: false,
            timezone: Some("Invalid/Timezone".to_string()),
            countdown: None,
        };
        cmd.execute();
    }

    #[test]
    fn test_execute_with_all_options() {
        let cmd = TimeCmd {
            format: "%H:%M:%S".to_string(),
            unix: true,
            timezone: Some("America/New_York".to_string()),
            countdown: None,
        };
        cmd.execute();
    }

    #[test]
    fn test_execute_with_countdown() {
        let cmd = TimeCmd {
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            unix: false,
            timezone: None,
            countdown: Some(1),
        };
        cmd.execute();
    }

    #[test]
    fn test_print_current_time_default_format() {
        let cmd = TimeCmd {
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            unix: false,
            timezone: None,
            countdown: None,
        };
        cmd.print_current_time();
    }

    #[test]
    fn test_print_current_time_custom_format() {
        let cmd = TimeCmd {
            format: "%H:%M".to_string(),
            unix: false,
            timezone: None,
            countdown: None,
        };
        cmd.print_current_time();
    }

    #[test]
    fn test_print_current_time_with_unix() {
        let cmd = TimeCmd {
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            unix: true,
            timezone: None,
            countdown: None,
        };
        cmd.print_current_time();
    }

    #[test]
    fn test_print_current_time_without_unix() {
        let cmd = TimeCmd {
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            unix: false,
            timezone: None,
            countdown: None,
        };
        cmd.print_current_time();
    }

    #[test]
    fn test_print_current_time_with_shanghai() {
        let cmd = TimeCmd {
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            unix: false,
            timezone: Some("Asia/Shanghai".to_string()),
            countdown: None,
        };
        cmd.print_current_time();
    }

    #[test]
    fn test_print_current_time_with_tokyo() {
        let cmd = TimeCmd {
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            unix: false,
            timezone: Some("Asia/Tokyo".to_string()),
            countdown: None,
        };
        cmd.print_current_time();
    }

    #[test]
    fn test_print_current_time_with_new_york() {
        let cmd = TimeCmd {
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            unix: false,
            timezone: Some("America/New_York".to_string()),
            countdown: None,
        };
        cmd.print_current_time();
    }

    #[test]
    fn test_print_current_time_with_invalid_tz() {
        let cmd = TimeCmd {
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            unix: false,
            timezone: Some("Not/A/Real/Zone".to_string()),
            countdown: None,
        };
        cmd.print_current_time();
    }

    #[test]
    fn test_print_current_time_with_empty_tz() {
        let cmd = TimeCmd {
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            unix: false,
            timezone: Some("".to_string()),
            countdown: None,
        };
        cmd.print_current_time();
    }

    #[test]
    fn test_run_countdown_1_second() {
        let cmd = TimeCmd {
            format: "%Y-%m-%d %H:%M:%S".to_string(),
            unix: false,
            timezone: None,
            countdown: None,
        };
        cmd.run_countdown(1);
    }

    #[test]
    fn test_tz_parsing() {
        assert!(Tz::from_str("Asia/Shanghai").is_ok());
        assert!(Tz::from_str("America/New_York").is_ok());
        assert!(Tz::from_str("Europe/London").is_ok());
        assert!(Tz::from_str("Invalid/Zone").is_err());
        assert!(Tz::from_str("").is_err());
    }
}