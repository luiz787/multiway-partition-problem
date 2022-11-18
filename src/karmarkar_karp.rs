use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Eq, Clone, Debug)]
pub struct Partition {
    pub subsets: Vec<Subset>,
    pub maximum_sum: u64,
    pub minimum_sum: u64,
}

#[derive(Eq, Clone, PartialEq, Debug)]
pub struct Subset {
    pub numbers: Vec<u64>,
    pub sum: i64
}

impl Subset {
    fn merge(mut s1: Subset, mut s2: Subset) -> Subset {
        let new_sum = s1.sum + s2.sum;
        s1.numbers.append(&mut s2.numbers);

        Subset {
            numbers: s1.numbers,
            sum: new_sum,
        }
    }
}

impl Partition {
    fn min_max_sum_difference(&self) -> u64 {
        self.maximum_sum - self.minimum_sum
    }

    fn merge(mut p1: Partition, mut p2: Partition) -> Partition {
        p1.subsets.sort_by(|s1, s2| s1.sum.cmp(&s2.sum));
        p2.subsets.sort_by(|s1, s2| s2.sum.cmp(&s1.sum));

        let mut new_subsets = Vec::new();

        while !p1.subsets.is_empty() {
            let s1 = p1.subsets.pop().expect("Expected to have subset");
            let s2 = p2.subsets.pop().expect("Expected to have subset");
            let new_subset = Subset::merge(s1, s2);

            new_subsets.push(new_subset);
        }

        let new_maximum_sum = new_subsets
            .iter()
            .max_by_key(|subset| subset.sum)
            .map(|subset| subset.sum as u64)
            .unwrap_or(u64::MIN);
        let new_minimum_sum = new_subsets
            .iter()
            .min_by_key(|subset| subset.sum)
            .map(|subset| subset.sum as u64)
            .unwrap_or(u64::MAX);

        Partition {
            subsets: new_subsets,
            maximum_sum: new_maximum_sum,
            minimum_sum: new_minimum_sum,
        }
    }
}

impl Ord for Partition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.min_max_sum_difference()
            .cmp(&other.min_max_sum_difference())
    }
}

impl PartialOrd for Partition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Partition {
    fn eq(&self, other: &Self) -> bool {
        self.subsets == other.subsets
    }
}

pub fn karmarkar_karp(nums: Vec<u64>, k: u64) -> Partition {
    let mut parts: BinaryHeap<Partition> = BinaryHeap::with_capacity(nums.len());
    for n in nums {
        parts.push(create_initial_partition(k, n));
    }

    while parts.len() > 1 {
        let (p1, p2) = select_top_two_partitions(&mut parts);
        let new_partition = Partition::merge(p1, p2);
        parts.push(new_partition);
    }

    parts.pop().expect("Unable to partition input")
}

fn create_initial_partition(k: u64, n: u64) -> Partition {
    let mut partition: Vec<Subset> = vec![Subset {
        numbers: vec![n],
        sum: n as i64,
    }];
    for _ in 1..k {
        partition.push(Subset {
            numbers: Vec::new(),
            sum: 0,
        });
    }

    Partition {
        subsets: partition,
        maximum_sum: n,
        minimum_sum: 0,
    }
}

fn select_top_two_partitions(parts: &mut BinaryHeap<Partition>) -> (Partition, Partition) {
    let first = parts.pop().expect("Expected top partition");
    let second = parts.pop().expect("Expected top partition");

    (first, second)
}
