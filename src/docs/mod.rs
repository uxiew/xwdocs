//! 文档管理模块

pub mod babel;
pub mod css;
pub mod documentation;
pub mod html;
pub mod javascript;
pub mod registry;
pub mod rust;
pub mod typescript;

use crate::docs::babel::BabelScraper;
pub use documentation::Documentation;
pub use registry::DocRegistry;

use crate::core::config::Config;
use crate::core::scraper::Scraper;
use std::error::Error;
use std::fs;
use std::path::Path;

/// 获取可用文档列表
pub fn get_available_docs() -> Vec<String> {
    vec![
        "babel".to_string(),
        "html".to_string(),
        "css".to_string(),
        "javascript".to_string(),
        "typescript".to_string(),
        "rust".to_string(),
    ]
}

/// 下载所有文档
pub async fn download_all_docs() -> Result<(), Box<dyn Error>> {
    println!("下载所有文档");

    // 获取所有可用文档并下载
    let docs = get_available_docs();
    for doc in docs {
        download_doc(&doc, "latest").await?;
    }

    Ok(())
}

/// 下载默认文档集
pub async fn download_default_docs() -> Result<(), Box<dyn Error>> {
    println!("下载默认文档");

    // 默认文档列表
    let default_docs = vec!["babel"];

    for doc in default_docs {
        download_doc(doc, "latest").await?;
    }

    Ok(())
}

/// 更新已安装的文档
pub async fn download_installed_docs() -> Result<(), Box<dyn Error>> {
    println!("更新已安装的文档");

    // 获取已安装的文档
    let installed_docs = get_installed_docs();
    for (doc, version) in installed_docs {
        download_doc(&doc, &version).await?;
    }

    Ok(())
}

/// 下载指定的文档列表
pub async fn download_specific_docs(docs: &[String]) -> Result<(), Box<dyn Error>> {
    println!("下载指定文档");

    for doc in docs {
        download_doc(doc, "latest").await?;
    }

    Ok(())
}

/// 下载单个文档
pub async fn download_doc(doc_name: &str, version: &str) -> Result<(), Box<dyn Error>> {
    println!("下载文档: {} (版本: {})", doc_name, version);

    let config = Config::default();

    // 确保文档目录存在
    let doc_dir = Path::new(&config.docs_path).join(doc_name);
    fs::create_dir_all(&doc_dir)?;

    // 根据文档类型执行不同的下载操作
    match doc_name {
        "babel" => {
            // 使用Babel抓取器下载文档
            let mut scraper = BabelScraper::new(&config.docs_path, version);
            scraper.run().await?;
        }
        // 添加其他文档类型的下载逻辑
        _ => {
            return Err(format!("未支持的文档类型: {}", doc_name).into());
        }
    }

    // 确保下载后处理
    println!("文档下载完成: {}", doc_name);

    Ok(())
}

/// 生成/抓取文档
pub async fn generate_doc(doc_name: &str, version: &str) -> Result<(), Box<dyn Error>> {
    println!("生成文档: {} (版本: {})", doc_name, version);

    let config = Config::default();

    match doc_name {
        "babel" => {
            let mut scraper = BabelScraper::new(&config.docs_path, version);
            scraper.run().await?;

            // 生成索引
            generate_doc_index(doc_name)?;
        }
        // 添加其他文档类型
        _ => {
            return Err(format!("未支持的文档类型: {}", doc_name).into());
        }
    }

    Ok(())
}

/// 生成文档索引
fn generate_doc_index(doc_name: &str) -> Result<(), Box<dyn Error>> {
    println!("生成文档索引: {}", doc_name);

    let config = Config::default();
    let doc_path = Path::new(&config.docs_path).join(doc_name);

    if !doc_path.exists() {
        return Err(format!("文档路径不存在: {:?}", doc_path).into());
    }

    // 读取条目数据
    let entries_file = doc_path.join("entries.json");
    if !entries_file.exists() {
        return Err(format!("条目文件不存在: {:?}", entries_file).into());
    }

    // 读取条目数据
    let entries_content = fs::read_to_string(&entries_file)?;
    let entries: serde_json::Value = serde_json::from_str(&entries_content)?;

    // 创建索引
    let mut index = serde_json::Map::new();

    // 添加索引元数据
    let mut index_meta = serde_json::Map::new();
    index_meta.insert(
        "doc".to_string(),
        serde_json::Value::String(doc_name.to_string()),
    );
    index.insert("entries".to_string(), serde_json::Value::Array(Vec::new()));

    // 处理entries数据，将其添加到索引中
    if let serde_json::Value::Array(entries_array) = entries {
        index.insert(
            "entries".to_string(),
            serde_json::Value::Array(entries_array),
        );
    }

    // 生成索引文件
    let index_file = doc_path.join("index.json");
    let index_content = serde_json::to_string_pretty(&index)?;
    fs::write(&index_file, index_content)?;

    println!("索引生成完成: {:?}", index_file);
    Ok(())
}

/// 生成单页
pub async fn generate_page(doc_name: &str, page_path: &str) -> Result<(), Box<dyn Error>> {
    println!("生成页面: {}/{}", doc_name, page_path);

    let config = Config::default();

    // 验证文档类型
    if !get_available_docs().contains(&doc_name.to_string()) {
        return Err(format!("未支持的文档类型: {}", doc_name).into());
    }

    // 确定页面URL或路径
    let page_url = match doc_name {
        "babel" => format!(
            "https://babeljs.io/docs/{}",
            page_path.trim_start_matches('/')
        ),
        // 添加其他文档类型的URL规则
        _ => return Err(format!("未支持的文档类型: {}", doc_name).into()),
    };

    // 抓取单个页面
    println!("抓取页面: {}", page_url);

    // 使用reqwest抓取页面内容
    let client = reqwest::Client::new();
    let response = client
        .get(&page_url)
        .header("User-Agent", "xwdoc/0.1.0")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("抓取页面失败: {} - {}", page_url, response.status()).into());
    }

    let content = response.text().await?;

    // 为文档创建输出目录
    let doc_dir = Path::new(&config.docs_path).join(doc_name);
    fs::create_dir_all(&doc_dir)?;

    // 解析出页面的相对路径并创建目录
    let page_rel_path = page_path.trim_start_matches('/').trim_end_matches('/');
    let page_dir = doc_dir.join(page_rel_path);
    fs::create_dir_all(&page_dir)?;

    // 将页面内容写入文件
    let output_file = page_dir.join("index.html");
    fs::write(&output_file, content)?;

    println!("页面抓取完成: {:?}", output_file);

    Ok(())
}

/// 打包文档
pub fn package_doc(doc_name: &str) -> Result<(), Box<dyn Error>> {
    println!("打包文档: {}", doc_name);

    let config = Config::default();
    let doc_path = Path::new(&config.docs_path).join(doc_name);

    if !doc_path.exists() {
        return Err(format!("文档路径不存在: {:?}", doc_path).into());
    }

    // 确保必要的文件存在
    let index_file = doc_path.join("index.json");
    if !index_file.exists() {
        return Err(format!("索引文件不存在: {:?}", index_file).into());
    }

    // 读取索引内容
    let index_content = fs::read_to_string(&index_file)?;
    let index: serde_json::Value = serde_json::from_str(&index_content)?;

    // 读取页面数据
    let mut pages_data = serde_json::Map::new();

    // 处理entries数组，提取页面内容
    if let Some(entries) = index.get("entries").and_then(|e| e.as_array()) {
        for entry in entries {
            if let Some(path) = entry.get("path").and_then(|p| p.as_str()) {
                // 确定页面文件路径
                let page_path = path.trim_start_matches('/');
                let page_dir = doc_path.join(page_path);
                let page_file = if page_dir.is_dir() {
                    page_dir.join("index.html")
                } else {
                    doc_path.join(format!("{}.html", page_path))
                };

                // 如果页面文件存在，读取内容
                if page_file.exists() {
                    if let Ok(content) = fs::read_to_string(&page_file) {
                        // 将页面内容添加到pages_data中
                        pages_data.insert(path.to_string(), serde_json::Value::String(content));
                    } else {
                        println!("警告: 无法读取页面文件: {:?}", page_file);
                    }
                } else {
                    println!("警告: 页面文件不存在: {:?}", page_file);
                }
            }
        }
    }

    // 创建完整的包数据
    let mut package_data = serde_json::Map::new();

    // 添加文档元数据
    let meta_file = doc_path.join("meta.json");
    if meta_file.exists() {
        if let Ok(meta_content) = fs::read_to_string(&meta_file) {
            if let Ok(meta_json) = serde_json::from_str::<serde_json::Value>(&meta_content) {
                package_data.insert("meta".to_string(), meta_json);
            }
        }
    }

    // 添加索引数据
    package_data.insert("index".to_string(), index);

    // 添加页面数据
    package_data.insert("pages".to_string(), serde_json::Value::Object(pages_data));

    // 添加打包时间戳
    package_data.insert(
        "created_at".to_string(),
        serde_json::Value::Number(serde_json::Number::from(chrono::Utc::now().timestamp())),
    );

    // 创建打包文件
    let package_file = doc_path.join("package.json");
    let package_content = serde_json::to_string_pretty(&serde_json::Value::Object(package_data))?;
    fs::write(&package_file, package_content)?;

    println!("文档打包完成: {:?}", package_file);

    Ok(())
}

/// 清理文档包
pub fn clean_docs() -> Result<(), Box<dyn Error>> {
    println!("清理文档包");

    let config = Config::default();
    let docs_path = Path::new(&config.docs_path);

    if !docs_path.exists() {
        println!("文档路径不存在，无需清理");
        return Ok(());
    }

    // 遍历所有文档目录
    if let Ok(entries) = fs::read_dir(docs_path) {
        for entry in entries.filter_map(Result::ok) {
            if entry.path().is_dir() {
                if let Some(doc_name) = entry.file_name().to_str() {
                    // 清理特定文档的临时文件和包文件
                    clean_doc_files(doc_name)?;
                }
            }
        }
    }

    println!("所有文档包清理完成");
    Ok(())
}

/// 清理特定文档的文件
fn clean_doc_files(doc_name: &str) -> Result<(), Box<dyn Error>> {
    println!("清理文档: {}", doc_name);

    let config = Config::default();
    let doc_path = Path::new(&config.docs_path).join(doc_name);

    if !doc_path.exists() {
        return Ok(());
    }

    // 需要清理的文件列表
    let files_to_clean = [
        "package.json",     // 打包文件
        "index.json.tmp",   // 临时索引文件
        "entries.json.tmp", // 临时条目文件
    ];

    for file in files_to_clean.iter() {
        let file_path = doc_path.join(file);
        if file_path.exists() {
            if let Err(e) = fs::remove_file(&file_path) {
                println!("警告: 无法删除文件 {:?}: {}", file_path, e);
            } else {
                println!("已删除: {:?}", file_path);
            }
        }
    }

    // 清理其他临时目录或文件，如果有的话
    // 这里可以添加特定文档的清理逻辑

    Ok(())
}

/// 生成文档清单
pub fn generate_manifest() -> Result<(), Box<dyn Error>> {
    println!("生成文档清单");

    let config = Config::default();
    let mut registry = DocRegistry::new();

    // 加载已有文档
    registry.load_from_disk(&config.docs_path)?;

    // 生成清单
    registry.generate_manifest(&config.docs_path)?;

    Ok(())
}

/// 获取已安装的文档
fn get_installed_docs() -> Vec<(String, String)> {
    let config = Config::default();
    let docs_path = Path::new(&config.docs_path);
    let mut result = Vec::new();

    // 读取docs目录
    if let Ok(entries) = fs::read_dir(docs_path) {
        for entry in entries.filter_map(Result::ok) {
            if entry.path().is_dir() {
                if let Some(doc_name) = entry.file_name().to_str() {
                    // 尝试读取版本信息
                    let version = get_doc_version(doc_name).unwrap_or_else(|| "latest".to_string());
                    result.push((doc_name.to_string(), version));
                }
            }
        }
    }

    // 如果没有找到任何文档，返回默认的Babel文档
    if result.is_empty() {
        result.push(("babel".to_string(), "7".to_string()));
    }

    result
}

/// 获取文档版本
fn get_doc_version(doc_name: &str) -> Option<String> {
    let config = Config::default();
    let doc_path = Path::new(&config.docs_path).join(doc_name);

    // 尝试读取版本文件
    let version_file = doc_path.join("version.txt");
    if version_file.exists() {
        if let Ok(version) = fs::read_to_string(version_file) {
            return Some(version.trim().to_string());
        }
    }

    // 根据文档类型返回默认版本
    match doc_name {
        "babel" => Some("7".to_string()),
        "html" => Some("5.3".to_string()),
        "css" => Some("3".to_string()),
        "javascript" => Some("ES6".to_string()),
        "typescript" => Some("4.5".to_string()),
        "rust" => Some("1.60".to_string()),
        _ => None,
    }
}
