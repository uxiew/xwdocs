//! 核心类型定义

/// 标识资源的唯一 ID
pub type ResourceId = String;

/// 文档的版本号
pub type Version = String;

/// 文档的别名
pub type Slug = String;

/// 文档的发布版本
pub type Release = String;

/// 文档的修改时间
pub type ModifiedTime = u64;

/// 大小（以字节为单位）
pub type Size = usize;