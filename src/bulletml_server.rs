use crate::{parse::BulletMLParser, BulletML};
use std::collections::HashMap;
use std::path;
use std::sync::Arc;

pub struct BulletMLServer {
    bmls: HashMap<String, Arc<BulletML>>,
}

impl BulletMLServer {
    pub fn new() -> Self {
        Self {
            bmls: HashMap::new(),
        }
    }

    pub fn load_file<P: AsRef<path::Path>>(
        &mut self,
        key: &str,
        path: P,
    ) -> anyhow::Result<Arc<BulletML>> {
        let parser = BulletMLParser::default();
        let parsed_bml = parser.parse_file(path)?;
        let rc_bml = Arc::new(parsed_bml);
        self.bmls.insert(key.to_string(), rc_bml.clone());

        Ok(rc_bml)
    }

    pub fn load_file_with_capacities<P: AsRef<path::Path>>(
        &mut self,
        key: &str,
        path: P,
        refs_capacity: usize,
        expr_capacity: usize
    ) -> anyhow::Result<Arc<BulletML>> {
        let parser = BulletMLParser::with_capacities(refs_capacity, expr_capacity);
        let parsed_bml = parser.parse_file(path)?;
        let rc_bml = Arc::new(parsed_bml);
        self.bmls.insert(key.to_string(), rc_bml.clone());

        Ok(rc_bml)
    }

    pub fn load(&mut self, key: &str, bytes: &[u8]) -> anyhow::Result<Arc<BulletML>> {
        let parser = BulletMLParser::default();
        let raw_bml_string = String::from_utf8(bytes.to_vec())?;
        let parsed_bml = parser.parse(&raw_bml_string)?;
        let rc_bml = Arc::new(parsed_bml);
        self.bmls.insert(key.to_string(), rc_bml.clone());

        Ok(rc_bml)
    }

    pub fn load_with_capacity(&mut self, key: &str, bytes: &[u8], refs_capacity: usize, expr_capacity: usize) -> anyhow::Result<Arc<BulletML>> {
        let parser = BulletMLParser::with_capacities(refs_capacity, expr_capacity);
        let raw_bml_string = String::from_utf8(bytes.to_vec())?;
        let parsed_bml = parser.parse(&raw_bml_string)?;
        let rc_bml = Arc::new(parsed_bml);
        self.bmls.insert(key.to_string(), rc_bml.clone());

        Ok(rc_bml)
    }

    pub fn get(&self, key: &str) -> Option<Arc<BulletML>> {
        match self.bmls.get(key) {
            Some(bml) => Some(bml.clone()),
            None => None,
        }
    }
}
