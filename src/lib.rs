pub use bit_operations::{BitOps,MutBitProxy};
pub use biter::{Biter,MutBiter};
use core::marker::PhantomData;
/// A BitSlice
pub struct BitSlice<ElementType,S> {
    slice:S,
    start_bit:u8,
    end_bit:u8,
    _life: PhantomData<ElementType>
}

impl <ElementType:BitOps,S:AsRef<[ElementType]>> BitSlice<ElementType,S> {
    pub fn bounds(&self,bit:usize) {if bit>=self.len() {panic!("Bit: {} is out of bounds as its greater than len: {}",bit,self.len())}}
    pub fn len(&self) -> usize {(self.slice.as_ref().len()-1)*ElementType::TYPE_BITS+(self.end_bit-self.start_bit) as usize+1}
    pub fn bits_idx(bit:usize) -> usize {bit/ElementType::TYPE_BITS}
    pub fn bits_bit(bit:usize) -> usize {bit%ElementType::TYPE_BITS}
    pub fn get(&self,bit:usize) -> bool {self.slice.as_ref()[Self::bits_idx(bit)].get_bit(Self::bits_bit(bit))}
    pub unsafe fn new_uncheked(slice:S, start_bit:usize, end_bit:usize) -> Self {Self {slice, start_bit:start_bit as u8, end_bit:end_bit as u8, _life: PhantomData}}
    pub fn new(slice:S) -> Self {unsafe {Self::new_uncheked(slice, 0, ElementType::TYPE_BITS-1 )}}
    pub fn iter<'short>(&'short self) -> Biter<'short,ElementType> {Biter::from(self)}
}

impl <ElementType:BitOps,S:AsMut<[ElementType]>+AsRef<[ElementType]>> BitSlice<ElementType,S> {
    pub fn set(&mut self,bit:usize, val:bool) {self.slice.as_mut()[Self::bits_idx(bit)].set_bit(Self::bits_bit(bit),val)}
    pub fn get_mut<'short>(&'short mut self, bit:usize) -> MutBitProxy<'short,ElementType> {self.slice.as_mut()[Self::bits_idx(bit)].mut_bit(Self::bits_bit(bit))}
    pub fn iter_mut<'short>(&'short mut self) -> MutBiter<'short,ElementType> {MutBiter::from(self)}
}

impl <'short, ElementType: BitOps,S:AsRef<[ElementType]> > From<&'short BitSlice<ElementType,S>> for Biter<'short,ElementType> {
    fn from(bs:&'short BitSlice<ElementType,S>) -> Self {
        unsafe {Self::new(bs.slice.as_ref() as *const [ElementType] as *const ElementType,bs.start_bit,bs.len())}
    }
}

impl <'short,  ElementType: BitOps,S:AsRef<[ElementType]> + AsMut<[ElementType]>> From<&'short mut BitSlice<ElementType,S>> for MutBiter<'short,ElementType> {
    fn from(bs:&'short mut BitSlice<ElementType,S>) -> Self {
        unsafe {Self::new(bs.slice.as_mut() as *mut [ElementType] as *mut ElementType,bs.start_bit,bs.len())}
    }
}