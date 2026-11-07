//! dust model — disk usage tool (du + rust).

#[derive(Debug, Clone)]
pub struct Dir {
    pub path: String,
    pub size: u64,
    pub depth: u32,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub dirs: Vec<Dir>,
}

impl Default for Model {
    fn default() -> Self { Self { dirs: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn scan(&mut self, path: &str, size: u64, depth: u32) {
        self.dirs.push(Dir { path: path.into(), size, depth });
    }

    pub fn sort_by_size(&mut self) {
        self.dirs.sort_by(|a, b| b.size.cmp(&a.size));
    }

    pub fn list(&self) -> &[Dir] { &self.dirs }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scan_sort() {
        let mut d = Model::new();
        d.scan("/tmp/a", 500, 1);
        d.scan("/tmp/b", 1000, 1);
        d.sort_by_size();
        assert_eq!(d.list()[0].size, 1000);
    }
}
