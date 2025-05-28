//! 图片处理过滤器
//! 将远程图片下载并转换为 Base64 编码的内嵌图片

use crate::core::error::{Error, Result};
use crate::core::filters::filter_base::FilterBase;
use crate::core::scraper::filter::Filter;
use crate::core::scraper::filter::FilterContext;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use image::GenericImageView;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::any::Any;
use std::io::Cursor;

/// 图片处理过滤器
///
/// 此过滤器会下载远程图片，将其转换为 Base64 编码后内嵌到 HTML 中
pub struct ImagesFilter {
    /// HTTP 客户端
    client: Client,
    /// 图片大小限制（字节）
    max_size: usize,
    /// 是否优化图片
    optimize_images: bool,
    /// 图片最大宽度
    max_width: Option<u32>,
}

impl ImagesFilter {
    /// 创建新的图片处理过滤器
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            max_size: 1024 * 300, // 默认 300KB
            optimize_images: true,
            max_width: None,
        }
    }

    /// 设置图片最大大小
    pub fn with_max_size(mut self, size: usize) -> Self {
        self.max_size = size;
        self
    }

    /// 设置是否优化图片
    pub fn with_optimize(mut self, optimize: bool) -> Self {
        self.optimize_images = optimize;
        self
    }

    /// 设置图片最大宽度
    pub fn with_max_width(mut self, width: u32) -> Self {
        self.max_width = Some(width);
        self
    }

    /// 下载图片并转换为 Base64
    fn download_image(&self, url: &str) -> Result<String> {
        // 发起请求下载图片
        let response = self // Removed mut
            .client
            .get(url)
            .send()
            .map_err(|e| Error::Doc(format!("Failed to download image from {}: {}", url, e)))?;

        // 检查状态码
        if !response.status().is_success() {
            return Err(Error::Doc(format!(
                "Failed to download image from {}: HTTP {}",
                url,
                response.status()
            )));
        }

        // 获取 Content-Type
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("image/jpeg")
            .to_string();

        // 获取图片数据
        let image_bytes = response
            .bytes()
            .map_err(|e| Error::Doc(format!("Failed to read image data from {}: {}", url, e)))?;

        // 检查图片大小
        if image_bytes.len() > self.max_size {
            // 如果启用了优化，尝试压缩图片
            if self.optimize_images {
                return self.optimize_image(url, &image_bytes, content_type);
            } else {
                return Err(Error::Doc(format!(
                    "Image too large: {} bytes (max: {} bytes)",
                    image_bytes.len(),
                    self.max_size
                )));
            }
        }

        // 对图片进行 Base64 编码
        let base64_str = STANDARD.encode(&image_bytes);

        // 返回 data URI
        Ok(format!("data:{};base64,{}", content_type, base64_str))
    }

    /// 优化图片
    fn optimize_image(
        &self,
        url: &str,
        image_bytes: &[u8],
        content_type: String,
    ) -> Result<String> {
        // 使用 image crate 加载图片
        let img = image::load_from_memory(image_bytes)
            .map_err(|e| Error::Doc(format!("Failed to decode image from {}: {}", url, e)))?;

        // 获取当前尺寸
        let (width, height) = img.dimensions();

        // 根据最大宽度调整图片大小
        let img = if let Some(max_width) = self.max_width {
            if width > max_width {
                let new_height = (height as f32 * (max_width as f32 / width as f32)) as u32;
                img.resize(max_width, new_height, image::imageops::FilterType::Lanczos3)
            } else {
                img
            }
        } else {
            img
        };

        // 写入内存
        let mut buffer = Cursor::new(Vec::new());

        // 根据原始图片格式保存为相同格式，默认为 JPEG
        let format = match content_type.as_str() {
            "image/png" => image::ImageFormat::Png,
            "image/gif" => image::ImageFormat::Gif,
            _ => image::ImageFormat::Jpeg, // 使用 JPEG 格式
        };

        // 保存为选择的格式
        img.write_to(&mut buffer, format)
            .map_err(|e| Error::Doc(format!("Failed to encode optimized image: {}", e)))?;

        // 获取压缩后的图片数据
        let optimized_bytes = buffer.into_inner();

        // 检查是否达到目标大小
        if optimized_bytes.len() > self.max_size {
            // 如果还是太大，返回错误
            return Err(Error::Doc(format!(
                "Optimized image still too large: {} bytes (max: {} bytes)",
                optimized_bytes.len(),
                self.max_size
            )));
        }

        // 对图片进行 Base64 编码
        let base64_str = STANDARD.encode(&optimized_bytes);

        // 返回 data URI
        Ok(format!("data:{};base64,{}", content_type, base64_str))
    }

    /// 检查是否为数据URL
    pub fn data_url_string(&self, str: &str) -> bool {
        str.starts_with("data:")
    }

    /// 检查是否为相对URL
    pub fn relative_url_string(&self, str: &str) -> bool {
        !str.contains("://") && !str.starts_with("data:") && !str.starts_with('#')
    }
}

impl FilterBase for ImagesFilter {}

impl Filter for ImagesFilter {
    fn apply(&self, html: &str, context: &mut FilterContext) -> Result<String> {
        // 解析 HTML
        let document = Html::parse_document(html);

        // 查找所有图片标签
        let selector =
            Selector::parse("img").map_err(|e| Error::Doc(format!("Invalid selector: {}", e)))?;

        // 创建结果
        let mut result = html.to_string();

        // 处理每个图片
        for img in document.select(&selector) {
            // 获取图片的源 URL
            let src = match img.value().attr("src") {
                Some(src) => src,
                None => continue, // 忽略没有 src 属性的图片
            };

            // 已经是 data URL 的跳过
            if self.data_url_string(src) {
                continue;
            }

            // 处理相对 URL
            let image_url = if self.relative_url_string(src) {
                format!("{}{}", context.base_url.trim_end_matches('/'), src)
            } else {
                src.to_string()
            };

            // 下载并转换图片
            match self.download_image(&image_url) {
                Ok(data_url) => {
                    // 获取原始 img 标签的 HTML
                    let img_html = img.html();

                    // 创建新的 img 标签替换 src
                    let new_img_html = img_html.replace(
                        &format!("src=\"{}\"", src),
                        &format!("src=\"{}\"", data_url),
                    );

                    // 替换 HTML 中的图片标签
                    result = result.replace(&img_html, &new_img_html);
                    println!("成功处理图片: {}", src);
                }
                Err(e) => {
                    eprintln!("图片处理失败: {}", e);
                    // 如果失败，保留原始图片
                }
            }
        }

        Ok(result)
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(Self {
            client: Client::new(),
            max_size: self.max_size,
            optimize_images: self.optimize_images,
            max_width: self.max_width,
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_images_filter_data_url() {
        let filter = ImagesFilter::new();
        let mut context = FilterContext::default();

        let html = r#"<img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCA...==" alt="Test">"#;
        let result = filter.apply(html, &mut context).unwrap();

        // 不应该修改已经是 data URL 的图片
        assert_eq!(html, result);
    }

    #[test]
    fn test_url_detection_methods() {
        let filter = ImagesFilter::new();

        // 测试 data URL 识别
        assert!(filter.data_url_string("data:image/png;base64,ABC"));
        assert!(!filter.data_url_string("https://example.com/image.png"));
        assert!(!filter.data_url_string("/images/icon.png"));

        // 测试相对 URL 识别
        assert!(filter.relative_url_string("/images/test.png"));
        assert!(filter.relative_url_string("../images/test.png"));
        assert!(filter.relative_url_string("images/test.png"));
        assert!(!filter.relative_url_string("https://example.com/image.png"));
        assert!(!filter.relative_url_string("http://localhost/image.png"));
        assert!(!filter.relative_url_string("data:image/png;base64,ABC"));
        assert!(!filter.relative_url_string("#section-1"));
    }

    #[test]
    fn test_relative_url_processing() {
        let filter = ImagesFilter::new();
        let mut context = FilterContext {
            base_url: "https://example.com".to_string(),
            ..FilterContext::default()
        };

        // 创建带有相对URL的HTML
        let html = r#"<img src="/images/test.png" alt="Test">"#;
        
        // 假设下载和转换功能，这里不实际执行
        // 主要测试相对URL处理逻辑
        assert!(filter.relative_url_string("/images/test.png"));
    }
}
