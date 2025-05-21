//! 过滤器注册表实现
//! 用于全局管理和访问过滤器

use crate::core::scraper::filter::Filter;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 过滤器创建函数类型
type FilterFactory = Box<dyn Fn() -> Box<dyn Filter> + Send + Sync>;

/// 全局过滤器注册表
pub struct FilterRegistry {
    /// 过滤器工厂函数映射
    factories: HashMap<String, FilterFactory>,
}

lazy_static! {
    /// 全局过滤器注册表实例
    static ref REGISTRY: Arc<Mutex<FilterRegistry>> = Arc::new(Mutex::new(FilterRegistry::new()));
}

impl FilterRegistry {
    /// 创建新的过滤器注册表
    fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    /// 获取全局注册表实例
    pub fn global() -> Arc<Mutex<FilterRegistry>> {
        REGISTRY.clone()
    }

    /// 注册过滤器工厂函数
    pub fn register<F, T>(&mut self, name: &str, factory: F)
    where
        F: Fn() -> T + 'static + Send + Sync,
        T: Filter + 'static,
    {
        let boxed_factory = Box::new(move || -> Box<dyn Filter> { Box::new(factory()) });
        self.factories.insert(name.to_string(), boxed_factory);
    }

    /// 创建过滤器实例
    pub fn create(&self, name: &str) -> Option<Box<dyn Filter>> {
        self.factories.get(name).map(|factory| factory())
    }

    /// 检查是否已注册过滤器
    pub fn contains(&self, name: &str) -> bool {
        self.factories.contains_key(name)
    }

    /// 获取所有已注册的过滤器名称
    pub fn filter_names(&self) -> Vec<String> {
        self.factories.keys().cloned().collect()
    }
}

/// 全局注册过滤器
pub fn register_filter<F, T>(name: &str, factory: F)
where
    F: Fn() -> T + 'static + Send + Sync,
    T: Filter + 'static,
{
    let mut registry = REGISTRY.lock().unwrap();
    registry.register(name, factory);
}

/// 全局创建过滤器实例
pub fn create_filter(name: &str) -> Option<Box<dyn Filter>> {
    let registry = REGISTRY.lock().unwrap();
    registry.create(name)
}

/// 全局检查是否已注册过滤器
pub fn contains_filter(name: &str) -> bool {
    let registry = REGISTRY.lock().unwrap();
    registry.contains(name)
}

/// 全局获取所有已注册的过滤器名称
pub fn filter_names() -> Vec<String> {
    let registry = REGISTRY.lock().unwrap();
    registry.filter_names()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scrapers::Filter;
    use std::any::Any;

    struct TestFilter;

    impl Filter for TestFilter {
        fn apply(
            &self,
            html: &str,
            _context: &crate::scrapers::filter::FilterContext,
        ) -> crate::core::error::Result<String> {
            Ok(html.to_string())
        }

        fn box_clone(&self) -> Box<dyn Filter> {
            Box::new(TestFilter)
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn test_filter_registry() {
        register_filter("test_filter", || TestFilter);

        assert!(contains_filter("test_filter"));
        assert!(!contains_filter("non_existent_filter"));

        let filter = create_filter("test_filter");
        assert!(filter.is_some());

        let filter_names = filter_names();
        assert!(filter_names.contains(&"test_filter".to_string()));
    }
}
