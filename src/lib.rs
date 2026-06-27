use core::marker::PhantomData;
use bit_operations::{BitOps,MutBitProxy};

pub struct BitSlice<'a,ElementType,Pointer,L> {
    pub start_ptr: Pointer,
    pub end_ptr: *const ElementType,
    pub start_bit:u8,
    pub end_bit: u8,
    _life: PhantomData<(&'a (),L)>
}

pub struct Biter<'a, ElementType,Pointer,L> {
    pub bit_slice: &'a BitSlice<'a,ElementType,Pointer,L>,
    pub current_bit:u8,
    pub current_pointer:Pointer
}

type Mutable<ElementType> = *mut ElementType;
type Immutable<ElementType> = *const ElementType;

pub trait BitSizes:BitOps {
    const TYPE_BITS: usize;
    const BIT_BITS:usize;
}

impl <ElementType:BitOps> BitSizes for ElementType {
    const TYPE_BITS: usize = (std::mem::size_of::<ElementType>()*8) as usize;
    const BIT_BITS:usize = Self::TYPE_BITS.ilog2() as usize;
}

impl<'a, ElementType: BitSizes,L> Iterator for Biter<'a, ElementType,Immutable<ElementType>,L> {
    type Item = bool; // Yields true/false for individual bits

    fn next(&mut self) -> Option<Self::Item> {
        let ptr_offsert = (self.bit_slice.end_bit+1) as usize/ElementType::TYPE_BITS;
        let end_bit = (self.bit_slice.end_bit+1)&((ElementType::TYPE_BITS-1 )as u8) ;
        if self.current_pointer>=unsafe{self.bit_slice.end_ptr.add(ptr_offsert)} && self.current_bit>=end_bit {return None}
        let bit = unsafe{(*self.current_pointer).get_bit(self.current_bit as usize)};
        self.current_bit += 1;
        self.current_pointer = unsafe {self.current_pointer.add(self.current_bit as usize>>ElementType::BIT_BITS)};
        self.current_bit&= ((ElementType::TYPE_BITS)-1) as u8; 
        Some(bit)
    }
}use std::ops::Index;

macro_rules! bitslice_mutability { //Shared methods
    ($(($pointer_type:ty, $ref_lock:ty)),*) => {
        $(
            impl <'a,ElementType:BitOps> BitSlice<'a,ElementType,$pointer_type,$ref_lock> {
                pub fn type_idx(bitdex:usize) -> usize {bitdex/(ElementType::TYPE_BITS)}
                pub fn type_bit(bitdex:usize) -> u8 {(bitdex%(ElementType::TYPE_BITS)) as u8}

                pub fn iter(&'a self) -> Biter<'a, ElementType,$pointer_type,$ref_lock> {
                    Biter {
                        bit_slice: self,
                        current_bit: self.start_bit,
                        current_pointer: self.start_ptr
                    }
                }

                pub fn get(&'a self, bitdex:usize) -> bool {
                    let (ptr,bit) = self.bitdex_to_valid_ptr_bit(bitdex);
                    unsafe { (*ptr).get_bit(bit as usize) }
                }

                pub unsafe fn bitdex_to_ptr_bit(&'a self, bitdex:usize) -> ($pointer_type,u8) {
                    (unsafe { self.start_ptr.add(Self::type_idx(bitdex)) },Self::type_bit(bitdex))
                }

                pub fn bitdex_to_valid_ptr_bit(&'a self, bitdex:usize) -> ($pointer_type,u8) {
                    let (ptr,bit) = unsafe { self.bitdex_to_ptr_bit(bitdex) };
                    self.ptr_bit_bounds(ptr,bit);
                    (ptr,bit)
                }

                pub fn ptr_bit_bounds(&'a self, ptr:Immutable<ElementType>, bit:u8) {
                    if ptr<self.start_ptr || ptr>self.end_ptr {panic!("Pointer OutOfBounds")}
                    else if ptr==self.start_ptr && bit<self.start_bit {panic!("Bit Lower OutOfBounds")}
                    else if ptr == self.end_ptr && bit>self.end_bit {panic!("Bit Upper OutOfBounds")}
                }
            }

            impl <'a,ElementType:BitOps> Index<usize> for BitSlice<'a,ElementType,$pointer_type,$ref_lock> {
                type Output = bool;

                fn index(&self, index: usize) -> &Self::Output {
                    if self.get(index) {&true} else {&false}
                }
            }
        )*
    }
}

bitslice_mutability!((Immutable<ElementType>,&'a ElementType), (Mutable<ElementType>,&'a mut ElementType));

impl <'a,ElementType:BitOps> BitSlice<'a,ElementType,Mutable<ElementType>,&'a mut ElementType> {
    pub fn set(&mut self, bitdex:usize, val:bool) {
        let (ptr,bit) = self.bitdex_to_valid_ptr_bit(bitdex);
        unsafe { (*ptr).set_bit(bit as usize, val) }
    }

    pub fn iter_mut(&'a self) -> Biter<'a, ElementType,Mutable<ElementType>,&'a mut ElementType> {
        Biter {
            bit_slice: self,
            current_bit: self.start_bit,
            current_pointer: self.start_ptr
        }
    }
}

pub trait BitCollection<'a,ElementType> {
    fn new_bitslice(&'a self) -> BitSlice<'a,ElementType,Immutable<ElementType>,&'a ElementType>;
    fn new_mut_bitslice(&'a mut self) -> BitSlice<'a,ElementType,Mutable<ElementType>,&'a mut ElementType>;
}

macro_rules! bitslice_collections {
    ( $( (collection:$collection:ty, generics:$($generic:tt)*) ),*) => {
        $(
            impl <$($generic)*> BitCollection<'a,ElementType> for $collection {
                
                fn new_bitslice(&'a self) -> BitSlice<'a,ElementType,Immutable<ElementType>,&'a ElementType> {
                    BitSlice {
                        start_ptr: &self[0] as *const ElementType,
                        end_ptr: &self[self.len()-1] as *const ElementType,
                        start_bit:0,
                        end_bit: ElementType::TYPE_BITS as u8-1,
                        _life: PhantomData
                    }
                }

                fn new_mut_bitslice(&'a mut self) -> BitSlice<'a,ElementType,Mutable<ElementType>,&'a mut ElementType> {
                    BitSlice {
                        start_ptr: &mut self[0] as *mut ElementType,
                        end_ptr: &self[self.len()-1] as *const ElementType,
                        start_bit:0,
                        end_bit: ElementType::TYPE_BITS as u8-1,
                        _life: PhantomData
                    }
                }
            }
        )*
    }
}

bitslice_collections!( (collection:Vec<ElementType>,generics:'a,ElementType:BitOps),(collection:[ElementType;N],generics:'a,ElementType:BitOps,const N:usize) );