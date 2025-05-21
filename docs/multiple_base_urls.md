# MultipleBaseUrls 模块

## 概述

`MultipleBaseUrls` 模块是 DevDocs Rust 项目的一部分，负责支持从多个基础 URL 抓取文档。该模块是对 `UrlScraper` 的扩展，允许一个爬虫实例处理多个相关但不同的基础 URL，例如不同版本或不同部分的文档。

## 功能特性

模块提供以下核心功能：

1. **添加多个基础 URL**：通过 `with_base_urls` 方法设置多个基础 URL
2. **获取所有基础 URL**：通过 `get_base_urls` 方法获取所有配置的基础 URL
3. **扩展初始 URL 列表**：初始抓取将包括所有基础 URL
4. **URL 处理判断**：判断 URL 是否应该处理时会检查所有基础 URL
5. **路径提取**：从多个基础 URL 中正确提取路径

## 使用示例

```rust
let scraper = UrlScraper::new("React", "18.0", "https://reactjs.org/docs/", "/output")
    .with_base_urls(vec![
        "https://reactjs.org/docs/".to_string(),
        "https://reactjs.org/tutorial/".to_string(),
        "https://reactjs.org/blog/".to_string(),
    ]);
```

## 实现细节

1. 第一个 URL 作为主基础 URL，但所有 URL 都用于决定是否处理链接
2. `get_initial_urls` 方法会返回所有基础 URL 的初始路径
3. `should_process_url` 方法会检查 URL 是否以任何一个基础 URL 开头
4. `url_to_path` 方法会正确地从任意基础 URL 中提取路径

## 与原始 Ruby 版本的对比

本实现忠实地遵循了原始 Ruby 版本的 `MultipleBaseUrls` 模块的设计理念，但做了一些 Rust 语言的适应性调整：

1. 使用 `Option<Vec<String>>` 替代 Ruby 中的 `@base_urls` 实例变量
2. 将模块功能直接整合到 `UrlScraper` 结构体中，而不是使用 mixin
3. 保留了相同的方法命名和功能设计

## 注意事项

- 确保所有基础 URL 都有相似的内容结构，以便过滤器能正确工作
- 第一个 URL 被视为主 URL，并用于某些默认操作
