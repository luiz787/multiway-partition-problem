use std::vec;

pub mod karmarkar_karp;
pub mod grasp;

mod common;

fn main() {
    let mut input = Vec::new();
    for _ in 0..1000 {
        input.push(1);
    }
    let result = karmarkar_karp::karmarkar_karp(input, 2);
    println!("{:#?}", result);
}
