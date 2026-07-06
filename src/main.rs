use bit_operations::{BitOps,MutBitProxy};
use biter::{Biter,MutBiter};
use core::marker::PhantomData;
/// A BitSlice
pub struct BitSlice<'a,ElementType,S> {
    pub slice:S,
    start_bit:u8,
    end_bit:u8,
    _life: PhantomData<&'a ElementType>
}

impl <'a,ElementType:BitOps,S:AsRef<[ElementType]>> BitSlice<'a,ElementType,S> {
    fn bounds(&self,bit:usize) {if bit>=self.len() {panic!("Bit: {} is out of bounds as its greater than len: {}",bit,self.len())}}
    fn len(&self) -> usize {(self.slice.as_ref().len()-1)*ElementType::TYPE_BITS+(self.end_bit-self.start_bit) as usize+1}
    fn bits_idx(bit:usize) -> usize {bit/ElementType::TYPE_BITS}
    fn bits_bit(bit:usize) -> usize {bit%ElementType::TYPE_BITS}
    fn get(&self,bit:usize) -> bool {self.slice.as_ref()[Self::bits_idx(bit)].get_bit(Self::bits_bit(bit))}
    unsafe fn new(slice:S, start_bit:usize, end_bit:usize) -> Self {Self {slice, start_bit:start_bit as u8, end_bit:end_bit as u8, _life: PhantomData}}
    fn iter<'short>(&'short self) -> Biter<'short,ElementType> {Biter::from(self)}
}

impl <'a,ElementType:BitOps,S:AsMut<[ElementType]>+AsRef<[ElementType]>> BitSlice<'a,ElementType,S> {
    fn set(&mut self,bit:usize, val:bool) {self.slice.as_mut()[Self::bits_idx(bit)].set_bit(Self::bits_bit(bit),val)}
    fn get_mut<'short>(&'short mut self, bit:usize) -> MutBitProxy<'short,ElementType> {self.slice.as_mut()[Self::bits_idx(bit)].mut_bit(Self::bits_bit(bit))}
    fn iter_mut<'short>(&'short mut self) -> MutBiter<'short,ElementType> {MutBiter::from(self)}
}

impl <'short,'long, ElementType: BitOps,S:AsRef<[ElementType]> > From<&'short BitSlice<'long,ElementType,S>> for Biter<'short,ElementType> {
    fn from(bs:&'short BitSlice<'long,ElementType,S>) -> Self {
        unsafe {Self::new(bs.slice.as_ref() as *const [ElementType] as *const ElementType,bs.start_bit,bs.len())}
    }
}

impl <'short, 'long, ElementType: BitOps,S:AsRef<[ElementType]> + AsMut<[ElementType]>> From<&'short mut BitSlice<'long,ElementType,S>> for MutBiter<'short,ElementType> {
    fn from(bs:&'short mut BitSlice<'long,ElementType,S>) -> Self {
        unsafe {Self::new(bs.slice.as_mut() as *mut [ElementType] as *mut ElementType,bs.start_bit,bs.len())}
    }
}

fn main() {
    println!("Hello, world!");
    let slice = &mut [1_u8,u8::MAX][0..=1];
    let mut bs = unsafe {BitSlice::new(slice,0,7)};
    assert_eq!(bs.get(0),true);
    {bs.get_mut(0);}
    assert_eq!(bs.get(1),false);
    bs.iter_mut().for_each(|bit|{println!("{}",*bit);});
    bs.iter().for_each(|bit|println!("{}",bit));
    println!("{}",bs.len());
    Biter::from_num(&6_u8).for_each(|bit|println!("{}",bit));
}
