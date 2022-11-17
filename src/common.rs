#[derive(Eq, Clone, Debug)]
pub struct Partition {
    // TODO: change to heap
    pub subsets: Vec<Subset>,
    pub maximum_sum: u64,
    pub minimum_sum: u64,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct Subset {
    pub numbers: Vec<u64>,
    pub sum: i64
}
