use crate::zpool::Zpool;

pub struct ZpoolComparison {
    source: Zpool,
    target: Zpool,
}

impl ZpoolComparison {

    pub fn new(source: Zpool, target: Zpool) -> Self {
        Self {
            source: source,
            target: target,
        }
    }

    pub fn print_table(&self) {
        println!("Compare: {} vs {}", self.source.len(), self.target.len());
    }

}
