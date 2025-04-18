use clap::Args;
use sysinfo::System;

/// 系统工具
#[derive(Args)]
pub struct SysCmd {
    
}

impl SysCmd {
    pub fn execute(&self) {
        let sys = System::new_all();
        let (used, total) = (
            sys.used_memory() as f64 / 1024f64.powi(3),
            sys.total_memory() as f64 / 1024f64.powi(3)
        );
        
        // 获取CPU品牌和核心数
        let cpu_count = sys.cpus().len();
        let cpu_name = sys.cpus().first().map(|c| c.brand()).unwrap_or("未知");

        println!("操作系统: {}",  System::name().unwrap());
        println!("系统版本: {}", System::os_version().unwrap());
        println!("内核版本: {}", System::kernel_version().unwrap());
        println!("内存: {:.2}/{:.2} GB  使用率: {:.2}%", used, total, used / total * 100.0);
        println!("CPU: {}c ({})",cpu_count, cpu_name);
    }
}