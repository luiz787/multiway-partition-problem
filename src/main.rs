use std::vec;

pub mod karmarkar_karp;
pub mod grasp;

mod common;

fn main() {
    let result = grasp::grasp(vec![8,7,6,5,4], 2, 200);
    println!("{:#?}", result);
}
