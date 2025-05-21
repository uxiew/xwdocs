//! 过滤器栈实现
//! 参考 Ruby 版本 filter_stack.rb 设计

use crate::core::scraper::filter::Filter;
use std::collections::HashMap;

/// 过滤器栈，类似有序集合，支持插入、替换和追加操作
pub struct FilterStack {
    /// 过滤器映射
    filters: Vec<(String, Box<dyn Filter>)>,
    /// 过滤器工厂函数映射
    factories: HashMap<String, Box<dyn Fn() -> Box<dyn Filter> + Send + Sync>>,
}

impl FilterStack {
    /// 创建新的过滤器栈
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
            factories: HashMap::new(),
        }
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

    /// 追加过滤器到栈尾部
    pub fn push(&mut self, name: &str) -> Result<(), String> {
        if let Some(factory) = self.factories.get(name) {
            let filter = factory();
            self.filters.push((name.to_string(), filter));
            Ok(())
        } else {
            Err(format!("未找到过滤器: {}", name))
        }
    }

    /// 在指定过滤器之前插入新过滤器
    pub fn insert_before(&mut self, index: &str, name: &str) -> Result<(), String> {
        if let Some(factory) = self.factories.get(name) {
            let filter = factory();

            if let Some(pos) = self.find_position(index) {
                self.filters.insert(pos, (name.to_string(), filter));
                Ok(())
            } else {
                Err(format!("未找到目标过滤器: {}", index))
            }
        } else {
            Err(format!("未找到过滤器: {}", name))
        }
    }

    /// 在指定过滤器之后插入新过滤器
    pub fn insert_after(&mut self, index: &str, name: &str) -> Result<(), String> {
        if let Some(factory) = self.factories.get(name) {
            let filter = factory();

            if let Some(pos) = self.find_position(index) {
                self.filters.insert(pos + 1, (name.to_string(), filter));
                Ok(())
            } else {
                Err(format!("未找到目标过滤器: {}", index))
            }
        } else {
            Err(format!("未找到过滤器: {}", name))
        }
    }

    /// 替换指定过滤器
    pub fn replace(&mut self, index: &str, name: &str) -> Result<(), String> {
        if let Some(factory) = self.factories.get(name) {
            let filter = factory();

            if let Some(pos) = self.find_position(index) {
                self.filters[pos] = (name.to_string(), filter);
                Ok(())
            } else {
                Err(format!("未找到目标过滤器: {}", index))
            }
        } else {
            Err(format!("未找到过滤器: {}", name))
        }
    }

    /// 直接添加命名过滤器（不通过工厂）
    pub fn push_filter(&mut self, name: &str, filter: Box<dyn Filter>) {
        self.filters.push((name.to_string(), filter));
    }

    /// 获取指定名称的过滤器
    pub fn get_filter(&self, name: &str) -> Option<&Box<dyn Filter>> {
        self.filters
            .iter()
            .find_map(|(n, f)| if n == name { Some(f) } else { None })
    }

    /// 获取所有过滤器名称
    pub fn filter_names(&self) -> Vec<String> {
        self.filters.iter().map(|(name, _)| name.clone()).collect()
    }

    /// 查找过滤器在栈中的位置
    fn find_position(&self, name: &str) -> Option<usize> {
        self.filters.iter().position(|(n, _)| n == name)
    }

    /// 获取过滤器列表
    pub fn filters(&self) -> Vec<Box<dyn Filter>> {
        self.filters.iter().map(|(_, f)| f.box_clone()).collect()
    }

    /// 检查是否包含指定过滤器
    pub fn contains(&self, name: &str) -> bool {
        self.filters.iter().any(|(n, _)| n == name)
    }

    /// 清空过滤器栈
    pub fn clear(&mut self) {
        self.filters.clear();
    }
}

impl Default for FilterStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scrapers::Filter;
    use std::any::Any;

    struct TestFilter(String);

    impl Filter for TestFilter {
        fn apply(
            &self,
            html: &str,
            context: &crate::scrapers::filter::FilterContext,
        ) -> crate::core::error::Result<String> {
            Ok(format!("{}_{}", html, self.0))
        }

        fn box_clone(&self) -> Box<dyn Filter> {
            Box::new(TestFilter(self.0.clone()))
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn test_filter_stack() {
        let mut stack = FilterStack::new();

        // 注册过滤器工厂
        stack.register("filter1", || TestFilter("1".to_string()));
        stack.register("filter2", || TestFilter("2".to_string()));
        stack.register("filter3", || TestFilter("3".to_string()));

        // 测试 push
        stack.push("filter1").unwrap();
        assert_eq!(stack.filters.len(), 1);

        // 测试 insert_after
        stack.insert_after("filter1", "filter3").unwrap();
        assert_eq!(stack.filters.len(), 2);
        assert_eq!(stack.filters[1].0, "filter3");

        // 测试 insert_before
        stack.insert_before("filter3", "filter2").unwrap();
        assert_eq!(stack.filters.len(), 3);
        assert_eq!(stack.filters[1].0, "filter2");

        // 测试 replace
        stack.replace("filter2", "filter1").unwrap();
        assert_eq!(stack.filters.len(), 3);
        assert_eq!(stack.filters[1].0, "filter1");

        // 测试 contains
        assert!(stack.contains("filter1"));
        assert!(!stack.contains("filter4"));

        // 测试 clear
        stack.clear();
        assert_eq!(stack.filters.len(), 0);
    }
}
