//! u9759u6001u6587u4ef6u670du52a1

use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

/// u63d0u4f9bu9759u6001u6587u4ef6
pub async fn serve_static_file(Path(_path): Path<String>) -> Response {
    // u5b9eu9645u5b9eu73b0u65f6u FF0Cu5c06u6839u636eu6587u4ef6u7c7bu578bu8bbeu7f6eu6b63u786eu7684 Content-Type
    // u5e76u5904u7406u7f13u5b58u548cu5176u4ed6u7684HTTP u5934
    
    // u4e3au4e86u5b89u5168u8d77u89c1uff0cu8bb0u5f97u9a8cu8bc1u8def u5f84u4e0du5305u542b.. u7b49u8defeuff0cu4ee5u907fu514du8def u5f84u904du5386u653bu51fb
    
    // u5360u4f4du7b26u54cdu5e94
    (StatusCode::NOT_FOUND, "Static file not found").into_response()
}

/// u60acu6d4bu5982u6709u54cdu5e94u6a21u677fu7684u5b58u5728
pub async fn serve_template(
    name: &str,
    context: impl serde::Serialize,
) -> Result<String, Box<dyn std::error::Error>> {
    // u5b9eu9645u5b9eu73b0u4e2du4f1au4f7fu7528u6a21u677fu5f15u64ceu5e93u6765u586bu5145u6a21u677f
    
    // u8fd9u91ccu8fd4u56deu4e00u4e2au4feeu526au8fc7u7684u7248u672c
    Ok(format!("<h1>Template: {}</h1><pre>{}</pre>", 
              name, 
              serde_json::to_string_pretty(&context)?))
}