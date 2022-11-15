use crate::common::{Subset, Partition};

impl Subset {
    fn merge(mut s1: Subset, mut s2: Subset) -> Subset {
        let new_sum = s1.sum + s2.sum;
        s1.numbers.append(&mut s2.numbers);
        
        Subset { numbers: s1.numbers, sum: new_sum }        
    }
}

impl Partition {
    fn min_max_sum_difference(&self) -> u64 {
        let maximum_sum = self.subsets.iter().max_by_key(|s| s.sum).expect("Partition has no subset").sum;
        let minimum_sum = self.subsets.iter().min_by_key(|s| s.sum).expect("Partition has no subset").sum;

        (maximum_sum - minimum_sum) as u64
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

        Partition { subsets: new_subsets }
    }
}

pub fn karmarkar_karp(nums: Vec<u64>, k: u64) -> Partition {
    let mut parts: Vec<Partition> = Vec::with_capacity(nums.len());
    for n in nums{
        parts.push(create_initial_partition(k, n));
    }

    while parts.len() > 1 {
        
        let (p1, p2) = select_top_two_partitions(&mut parts);
        
        let p1_index = parts.iter().position(|p| p == &p1).expect("Partition should be in the list");
        parts.remove(p1_index);
        
        let p2_index = parts.iter().position(|p| p == &p2).expect("Partition should be in the list");
        parts.remove(p2_index);

        let new_partition = Partition::merge(p1, p2);
        parts.push(new_partition);
    }

    parts.into_iter().take(1).last().expect("Unable to partition input")
}

fn create_initial_partition(k: u64, n: u64) -> Partition {
    let mut partition: Vec<Subset> = vec![Subset { numbers: vec![n], sum: n as i64 }];
    for _ in 1..k {
        partition.push(Subset { numbers: Vec::new(), sum: 0 });
    }

    Partition { subsets: partition }
}

fn select_top_two_partitions(parts: &Vec<Partition>) -> (Partition, Partition) {
    let mut as_vector: Vec<Partition> = parts.clone().into_iter().collect();
    as_vector.sort_by(|a, b| b.min_max_sum_difference().cmp(&a.min_max_sum_difference()));

    let top_two: Vec<Partition> = as_vector.into_iter().take(2).collect::<Vec<Partition>>();

    (top_two[0].clone(), top_two[1].clone())
}
