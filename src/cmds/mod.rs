//! 命令模块
//! 包含所有子命令的实现

pub mod uuid;
pub mod pw;

pub use uuid::UuidCmd;
pub use pw::PwCmd; 