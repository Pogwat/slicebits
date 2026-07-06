#![no_std]
#[doc = include_str!("../README.md")]
pub use bit_operations::{BitOps,MutBitProxy};
pub use biter::{Biter,MutBiter};
use core::marker::PhantomData;
/// A BitSlice
pub struct BitSlice<ElementType,S> {
    /// The Slice being refenced as bits
    slice:S,
    /// The start bit
    start_bit:u8,
    /// The end bit
    end_bit:u8,
    _life: PhantomData<ElementType>
}

/// Methods for Immutable BitSlice
impl <ElementType:BitOps,S:AsRef<[ElementType]>> BitSlice<ElementType,S> {
    ///Check if a git index is in BitSlice, if not panics
    pub fn bounds(&self,bit:usize) {if bit>=self.len() {panic!("Bit: {} is out of bounds as its greater than len: {}",bit,self.len())}}
    ///Number of bits in BitlSlice
    pub fn len(&self) -> usize {(self.slice.as_ref().len()-1)*ElementType::TYPE_BITS+(self.end_bit-self.start_bit) as usize+1}
    /// Global Bit Index to bit positon in a element
    pub fn bits_idx(bit:usize) -> usize {bit/ElementType::TYPE_BITS}
    /// Global Bit Index to a element index
    pub fn bits_bit(bit:usize) -> usize {bit%ElementType::TYPE_BITS}
    /// Get bit by global index
    pub fn get(&self,bit:usize) -> bool {self.slice.as_ref()[Self::bits_idx(bit)].get_bit(Self::bits_bit(bit))}
    /// Get a new BitSlice from A type that can be sliced (impls AsRef<[Elementype]>), a start bit, and a end bit (unsafe)
    pub unsafe fn new_uncheked(slice:S, start_bit:usize, end_bit:usize) -> Self {Self {slice, start_bit:start_bit as u8, end_bit:end_bit as u8, _life: PhantomData}}
    /// Get a new BitSlice from A type that can be sliced (impls AsRef<[Elementype]>)
    pub fn new(slice:S) -> Self {unsafe {Self::new_uncheked(slice, 0, ElementType::TYPE_BITS-1 )}}
    /// Iterate over a BitSlice, yields bools
    pub fn iter<'short>(&'short self) -> Biter<'short,ElementType> {Biter::from(self)}
}

/// Methods for Mutable BitSlice
impl <ElementType:BitOps,S:AsMut<[ElementType]>+AsRef<[ElementType]>> BitSlice<ElementType,S> {
    /// Set a bit by global index in a BitSlice
    pub fn set(&mut self,bit:usize, val:bool) {self.slice.as_mut()[Self::bits_idx(bit)].set_bit(Self::bits_bit(bit),val)}
    /// Get Mutable refrence to Bit (porxy struct: MutBitProxy), REF MUST BE DROPPED FOR BIT TO UPDATE. DROP UPDATES!!!
    pub fn get_mut<'short>(&'short mut self, bit:usize) -> MutBitProxy<'short,ElementType> {self.slice.as_mut()[Self::bits_idx(bit)].mut_bit(Self::bits_bit(bit))}
    /// Mutably Iterate over a BitSlice, yields MutBitProxy that can be Derefed to a bool
    pub fn iter_mut<'short>(&'short mut self) -> MutBiter<'short,ElementType> {MutBiter::from(self)}
}

///Biter from A Immutable BitSlice ref
impl <'short,ElementType: BitOps,S:AsRef<[ElementType]> > From<&'short BitSlice<ElementType,S>> for Biter<'short,ElementType> {
    fn from(bs:&'short BitSlice<ElementType,S>) -> Self {
        unsafe {Self::new(bs.slice.as_ref() as *const [ElementType] as *const ElementType,bs.start_bit,bs.len())}
    }
}

///MutBiter from A Mutable BitSlice ref
impl <'short,ElementType: BitOps,S:AsRef<[ElementType]> + AsMut<[ElementType]>> From<&'short mut BitSlice<ElementType,S>> for MutBiter<'short,ElementType> {
    fn from(bs:&'short mut BitSlice<ElementType,S>) -> Self {
        unsafe {Self::new(bs.slice.as_mut() as *mut [ElementType] as *mut ElementType,bs.start_bit,bs.len())}
    }
}