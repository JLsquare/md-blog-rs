use std::time::SystemTime;
use std::collections::HashMap;

pub struct CachedMarkdown {
    pub html: String,
    pub last_modified: SystemTime,
}

pub struct MarkdownCache {
    pub cache: HashMap<String, CachedMarkdown>,
}

impl MarkdownCache {
    pub fn new() -> Self {
        MarkdownCache {
            cache: HashMap::new(),
        }
    }
}