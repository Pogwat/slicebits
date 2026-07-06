#[test]
fn main() {
    use slicebits::{BitSlice,Biter};
    println!("Hello, world!");
    let slice = &mut [1_u8,u8::MAX][0..=1];
    let mut bs = BitSlice::new(slice);
    assert_eq!(bs.get(0),true);
    {bs.get_mut(0);}
    assert_eq!(bs.get(1),false);
    bs.iter_mut().for_each(|bit|{println!("{}",*bit);});
    bs.iter().for_each(|bit|println!("{}",bit));
    println!("{}",bs.len());
    Biter::from_num(&6_u8).for_each(|bit|println!("{}",bit));
}

#[test]
fn bitslice_biter() {
    use slicebits::BitSlice;
    let array: [u8;2] = [1,2];
    let bitslice = BitSlice::new(&array);
    let mut set_bits=0;
    bitslice.iter().for_each(|bit| set_bits+=bit as usize);
    assert_eq!(set_bits,2);

    let mut array = array;
    set_bits=0;
    let mut mbitslice = BitSlice::new(&mut array);
    mbitslice.iter_mut().for_each(|mut bit| *bit = true);
    mbitslice.iter().for_each(|bit| set_bits+=bit as usize);
    assert_eq!(set_bits,2*8);
}

#[test]
fn bitslice_set_get() {
    use slicebits::BitSlice;
    let mut array: [u8;2] = [1,2];

    {
    let mut bitslice = BitSlice::new(&mut array);
    assert_eq!(bitslice.get(0),true);
    assert_eq!(bitslice.get(1),false);
    assert_eq!(bitslice.get(7+2),true);
    bitslice.set(0,false);
    bitslice.set(7+2,false);
    assert_eq!(bitslice.get(0),false)
    }

    assert_eq!(array.iter().sum::<u8>(),0);
    assert_eq!(array[0]+ array[1],0);
    {
    let mut bitslice = BitSlice::new(&mut array);
    bitslice.set(7+2,true);
    }
    assert_eq!(array[1],2);
}

#[test]
fn bitslice_get_mut_len() {
    use slicebits::BitSlice;
    let mut array: [u8;2] = [1,2];
    let mut bitslice = BitSlice::new(&mut array);
    assert_eq!(bitslice.len(),2*8);
    {
    let mut bit = bitslice.get_mut(0);
    assert_eq!(*bit,true);
    *bit=false;
    }
    assert_eq!(array[0],0);
}