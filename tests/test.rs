#[test]
fn main() {
    println!("hi");
}

use bitslices::*;

#[test]
fn vec_bitslice() {
    let mut vec:Vec<u8> = Vec::new();
    vec.push(1);
    let vec_bs = vec.new_bitslice();
    assert_eq!(vec_bs.get(0),true);
    assert_eq!(vec_bs.get(1),false);
}