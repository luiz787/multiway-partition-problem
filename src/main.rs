use std::vec;

pub mod karmarkar_karp;

fn main() {
    let result = karmarkar_karp::karmarkar_karp(vec![8,7,6,5,4], 2);
    println!("{:#?}", result);
}
