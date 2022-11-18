use std::vec;

pub mod karmarkar_karp;
pub mod grasp;

fn main() {
    let mut input = Vec::new();
    for i in 0..1000 {
        input.push(i);
    }
    let result = grasp::grasp(input, 3, 50);
    // let result = karmarkar_karp::karmarkar_karp(input, 3);
    //let result = grasp::greedy_heuristic(&input, 3);
    println!("{:#?}", result);
}
