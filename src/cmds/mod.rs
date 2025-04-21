//! 命令模块
//! 包含所有子命令的实现

pub mod uuid;
pub mod pw;
pub mod ip;
pub mod sys;
pub mod time;  // 新增

pub use uuid::UuidCmd;
pub use pw::PwCmd; 
pub use ip::IpCmd;
pub use sys::SysCmd;
pub use time::TimeCmd;  // 新增