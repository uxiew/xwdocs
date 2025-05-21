# DevDocs Rust — API Documentation Browser

DevDocs Rust is a Rust implementation of [DevDocs](https://devdocs.io), an API documentation browser that combines multiple developer documentations in a clean and organized web UI with instant search, offline support, mobile version, dark theme, keyboard shortcuts, and more.

## Quick Start

```
1. 子命令结构建议（与 Ruby 版 Thor 保持一致）
Server
xwdoc server
启动 Web 服务器
Docs
xwdoc docs list
列出可用文档
xwdoc docs download [docs...] [--default|--installed|--all]
下载文档
xwdoc docs manifest
生成 manifest 文件
xwdoc docs generate [doc]
生成/抓取文档
xwdoc docs page [doc] [page]
生成/抓取单页
xwdoc docs package [doc]
打包文档
xwdoc docs clean
删除文档包
Console
xwdoc console
启动 REPL
xwdoc console docs
启动 Docs 模块 REPL
Test
xwdoc test all
运行所有测试
xwdoc test docs
运行 Docs 测试
xwdoc test app
运行 App 测试
Assets
xwdoc assets compile
编译前端资源
xwdoc assets clean
清理旧资源
2. clap 子命令结构建议
建议将 Commands 拆分为多级子命令（如 Docs/Console/Test/Assets），并为每个子命令实现对应的参数和帮助说明。

3. 兼容性与扩展
保持参数、命令名、帮助信息与 Ruby 版一致。
支持 --help、-h、--version。
支持配置文件、环境变量（如 XWDOC_DATA_DIR）。
4. 下一步建议
重构 Commands，实现多级子命令（如 Docs/Console/Test/Assets）。
实现 docs download、docs generate、docs page、docs package、docs clean 等子命令。
在 README.md 中补充 Rust 版命令用法示例，方便迁移。
```

```sh
# Clone the repository
git clone https://github.com/yourusername/devdocs_rust.git && cd devdocs_rust

# Build the project
cargo build

# Run the server
cargo run server

# Scrape HTML documentation
cargo run scrape --name html --version latest --url https://developer.mozilla.org/en-US/docs/Web/HTML

# View available documentation list
cargo run list
```

## Project Structure

The project is based on a modular design in Rust, divided into the following main components:

1. **Core Components (core)**: Provides configuration, error handling, and basic type definitions.

2. **Documentation Components (docs)**: Manages document metadata and index information.

3. **Scrapers (scrapers)**: Scrapes documentation content from different sources (web or local files).

   - URL scraper: Downloads documentation from the web
   - File scraper: Reads documentation from the local file system
   - Specific document type scrapers: For HTML, CSS, JavaScript, Rust, etc.

4. **Filters (filters)**: Processes and cleans HTML content, including:

   - HTML filters: Cleans HTML structure, standardizes URLs, etc.
   - Text filters: Processes and extracts content

5. **Storage (storage)**: Stores and retrieves document files.

6. **Web Server (web)**: Provides web interface, including routes, handlers, and static file services.

7. **Command Line Tools (cli)**: Provides command line interaction, including the following commands:
   - `server`: Starts the web server
   - `scrape`: Scrapes specific documentation
   - `manifest`: Generates document manifest
   - `list`: Lists available documentation

## Features

- **Multiple Document Type Support**: HTML, CSS, JavaScript, Rust and more programming languages and framework documentation
- **Instant Search**: Efficient search engine supporting full text search
- **Offline Mode**: Support for offline access to all documentation
- **Clean Interface**: Simple modern web interface design
- **Extensibility**: Modular design, easy to extend with new document types
- **High Performance**: Implemented in Rust, maximizing performance and memory efficiency

## Implemented Modules

### Document Management

- `Documentation`: Structure representing a single document, including metadata and path information
- `DocRegistry`: Manages all available documentation, supports loading and finding documents

### Scrapers

- `Scraper` trait: Defines common interface for scrapers
- `UrlScraper`: Scrapes documentation from URLs with:
  • URL extraction and queueing
  • Breadth-first traversal of documentation sites
  • Content filtering and saving
- `FileScraper`: Reads documentation from local file system with:
  • HTML file discovery
  • Content filtering
  • Path normalization
- Specific language scrapers: Implemented for HTML, CSS, JavaScript and Rust

### Filters

- `Filter` trait: Defines content filter interface
- `Pipeline`: Pipeline structure, processing content through multiple filters
  • Sequential filter application
  • Configurable filter chains
- HTML filters:
  • `HtmlCleanerFilter`: Removes unnecessary tags and attributes
  • `UrlNormalizerFilter`: Standardizes URLs for offline viewing
  • Other specialized filters for different document types

### Storage

- `Store` trait: Defines storage interface
- `FileStore`: File system based storage implementation

### Web Server

- `Server`: Manages web server configuration and startup
- `routes`: Defines API routes
- `handlers`: Handles HTTP requests

### Command Line Interface

- `Cli`: Command line parser implemented with clap
- Supported commands: `server`, `scrape`, `manifest`, `list`

## Testing

The project includes both unit tests and integration tests:

```sh
# Run all tests
cargo test

# Run specific tests
cargo test url_scraper
```

## Extension Development

If you wish to extend the project, here are some common development tasks:

### Adding a New Document Scraper

1. Create a new scraper module in the `src/scrapers/docs` directory
2. Implement the `Scraper` trait
3. Add the new document type in the `scrape` function in `lib.rs`

### Adding a New Filter

1. Create a new filter in the `src/filters` directory
2. Implement the `Filter` trait
3. Use the filter in the `Pipeline`

### Improving the Web Interface

1. Update routes in `src/web/routes.rs`
2. Add new request handlers in `src/web/handlers.rs`

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the project
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push the branch (`git push origin feature/amazing-feature`)
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

- [DevDocs](https://devdocs.io) - The original project and source of inspiration
- All contributors and developers
