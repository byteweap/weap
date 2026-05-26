use clap::Args;
use sysinfo::System;

/// 系统工具
#[derive(Args)]
pub struct SysCmd {}

impl SysCmd {
    pub fn execute(&self) {
        let sys = System::new_all();
        let (used, total) = (
            sys.used_memory() as f64 / 1024f64.powi(3),
            sys.total_memory() as f64 / 1024f64.powi(3),
        );

        let cpu_count = sys.cpus().len();
        let cpu_name = sys.cpus().first().map(|c| c.brand()).unwrap_or("未知");

        let name = System::name().unwrap_or_else(|| "未知".to_string());
        let os_version = System::os_version().unwrap_or_else(|| "未知".to_string());
        let kernel = System::kernel_version().unwrap_or_else(|| "未知".to_string());

        let usage = if total > 0.0 { used / total * 100.0 } else { 0.0 };

        println!("操作系统: {}", name);
        println!("系统版本: {}", os_version);
        println!("内核版本: {}", kernel);
        println!("内存: {:.2}/{:.2} GB  使用率: {:.2}%", used, total, usage);
        println!("CPU: {}c ({})", cpu_count, cpu_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_basic() {
        let cmd = SysCmd {};
        cmd.execute();
    }

    #[test]
    fn test_system_info_not_empty() {
        let sys = System::new_all();
        let name = System::name().unwrap_or_else(|| "未知".to_string());
        let os_version = System::os_version().unwrap_or_else(|| "未知".to_string());
        let kernel = System::kernel_version().unwrap_or_else(|| "未知".to_string());
        let cpu_count = sys.cpus().len();

        assert!(!name.is_empty());
        assert!(!os_version.is_empty());
        assert!(!kernel.is_empty());
        assert!(cpu_count > 0);
    }

    #[test]
    fn test_memory_usage_calculation() {
        let sys = System::new_all();
        let used = sys.used_memory() as f64 / 1024f64.powi(3);
        let total = sys.total_memory() as f64 / 1024f64.powi(3);

        if total > 0.0 {
            let usage = used / total * 100.0;
            assert!(usage >= 0.0 && usage <= 100.0);
        } else {
            // total == 0.0 的情况（理论上在真实系统上不会发生）
            assert_eq!(total, 0.0);
        }
    }

    #[test]
    fn test_cpu_name_not_empty() {
        let sys = System::new_all();
        let cpu_name = sys.cpus().first().map(|c| c.brand()).unwrap_or("未知");
        assert!(!cpu_name.is_empty());
    }
}
