//! 存储接口

use crate::core::error::Result;

/// 存储接口特质
pub trait Store {
    /// 读取一个文件
    fn read(&self, path: &str) -> Result<String>;

    /// 写入一个文件
    fn write(&self, path: &str, content: &str) -> Result<()>;

    /// 检查文件是否存在
    fn exists(&self, path: &str) -> Result<bool>;

    /// 列出文件
    fn list(&self, dir: &str) -> Result<Vec<String>>;

    /// 删除文件
    fn delete(&self, path: &str) -> Result<()>;

    /// 获取文件大小
    fn size(&self, path: &str) -> Result<usize>;
}
