#[derive(Eq, Clone, PartialEq, Debug)]
pub struct Partition {
    pub subsets: Vec<Subset>
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct Subset {
    pub numbers: Vec<u64>,
    pub sum: i64
}
