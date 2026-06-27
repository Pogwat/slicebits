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
    let mut len=0;
    vec_bs.iter().enumerate().for_each(|(bitdex,bit)| {
        println!("{}",bitdex);
        if bitdex==0 {assert_eq!(bit,true)}
        if bitdex==1 {assert_eq!(bit,false)}
        if bitdex==7 {assert_eq!(bit,false)}
        if bitdex==8 {panic!("8 is Out of bounds")}
        len+=1
    });
    assert_eq!(len,8);
}

#[test]
#[should_panic]
fn vec_bitslice_oob_bitdex_get() {
    let mut vec:Vec<u8> = Vec::new();
    vec.push(1);
    let vec_bs = vec.new_bitslice();
    vec_bs.get(8);
}

#[test]
#[should_panic]
fn vec_bitslice_oob_bitdex_set() {
    let mut vec:Vec<u8> = Vec::new();
    vec.push(1);
    let mut vec_bs = vec.new_mut_bitslice();
    vec_bs.set(8,true);
}

#[test]
fn vec_mut_bitslice() {
    let mut vec:Vec<u8> = Vec::new();
    vec.push(1);
    let mut vec_bs = vec.new_mut_bitslice();
    vec_bs.set(7,true);
    assert_eq!(vec_bs.get(7),true);
}

// #[test]
// #[should_panic]
// fn vec_bitslice_lock() {
//     let mut vec:Vec<u8> = vec![1];
//     let vec_bs = vec.new_bitslice();
//     vec.push(0);
//     vec_bs.get(0);
// }

#[test]
fn slicing_index() {
    let vec:Vec<u8> = vec![1];
    let vec_bs = vec.new_bitslice();
    assert_eq!(vec_bs[0],true);
    assert_eq!(vec_bs[1],false);
}

