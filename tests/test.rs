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
    assert_eq!(vec_bs.get(2),false);
    assert_eq!(vec_bs.get(3),false);
    assert_eq!(vec_bs.get(4),false);
    assert_eq!(vec_bs.get(5),false);
    assert_eq!(vec_bs.get(6),false);
    assert_eq!(vec_bs.get(7),false);
    println!("{}",vec_bs.end_bit);
    vec_bs.iter().enumerate().for_each(|(bitdex,bit)| {
        println!("{}",bitdex);
        if bitdex==0 {assert_eq!(bit,true)}
        if bitdex==1 {assert_eq!(bit,false)}
        if bitdex==8 {panic!("8 is Out of bounds")}
    }
    )
}