use rand::prelude::IteratorRandom;
use rand::thread_rng;
use std::collections::BinaryHeap;

use crate::common::{Partition, Subset};

const CANDIDATES_LIST_SIZE: usize = 5;

impl Partition {
    fn solution_quality(&self) -> u64 {
        self.subsets
            .iter()
            .max_by(|s1, s2| s1.sum.cmp(&s2.sum))
            .expect("Expected partition to be non-empty")
            .sum as u64
    }
}

pub fn greedy_heuristic(nums: &Vec<u64>, k: u64) -> Partition {
    let mut rng = thread_rng();
    let mut parts: Vec<Subset> = Vec::new();
    let mut s = BinaryHeap::new();
    for _ in 0..(k as usize) {
        parts.push(Subset {
            numbers: Vec::new(),
            sum: 0,
        });
    }
    for n in nums {
        s.push(n);
    }

    while !s.is_empty() {
        let mut candidates = Vec::new();
        for _ in 0..CANDIDATES_LIST_SIZE.min(s.len()) {
            candidates.push(s.pop().expect("Expected heap to be non-empty"))
        }

        let index = (0..candidates.len())
            .choose(&mut rng)
            .expect("Expected candidates list to be non-empty");
        parts.sort_by(|p1, p2| {
            p1.numbers
                .iter()
                .sum::<u64>()
                .cmp(&p2.numbers.iter().sum::<u64>())
        });

        parts[0].numbers.push(*candidates[index]);
        parts[0].sum += *candidates[index] as i64;

        for i in 0..CANDIDATES_LIST_SIZE.min(s.len() + candidates.len()) {
            if i == index {
                continue;
            }
            s.push(candidates[i]);
        }
    }

    Partition { subsets: parts }
}

pub fn grasp(nums: Vec<u64>, k: u64, max_iter: u64) -> Partition {
    let mut best: Option<Partition> = Option::None;
    let mut i = 0;
    while i < max_iter {
        println!("Grasp, iter {}", i);
        let solution = greedy_heuristic(&nums, k);
        let solution = local_search(solution);

        let is_better = match &best {
            Some(partition) => solution.solution_quality() < partition.solution_quality(),
            None => true,
        };

        if is_better {
            best = Option::Some(solution);
        }
        i += 1
    }

    best.expect("No solution found")
}

fn local_search(solution: Partition) -> Partition {
    let mut curr = solution;
    loop {
        let neighbor = best_neighbor(&curr);
        let neighbor_is_better = neighbor.solution_quality() < curr.solution_quality();

        if !neighbor_is_better {
            return curr;
        }
        curr = neighbor
    }
}

fn best_neighbor(solution: &Partition) -> Partition {
    let mut neighbors = Vec::new();
    for i in 0..(solution.subsets.len() - 1) {
        for j in 0..solution.subsets[i].numbers.len() {
            for k in 0..solution.subsets[i + 1].numbers.len() {
                let mut current_neighbor = solution.clone();
                let tmp = solution.subsets[i].numbers[j];
                let tmp2 = solution.subsets[i + 1].numbers[k];
                
                // Swap the j-th element from the i-th subset with the k-th element from the (i+1)-th subset
                current_neighbor.subsets[i].numbers[j] = solution.subsets[i + 1].numbers[k];
                current_neighbor.subsets[i].sum += (tmp2 as i64) - (tmp as i64);
                current_neighbor.subsets[i + 1].numbers[k] = tmp;
                current_neighbor.subsets[i + 1].sum += (tmp as i64) - (tmp2 as i64);

                neighbors.push(current_neighbor);
            }
        }
    }

    neighbors
        .into_iter()
        .min_by(|p1, p2| p1.solution_quality().cmp(&p2.solution_quality()))
        .expect("Expected partition to have neighbors")
}