use core::marker::PhantomData;
use bit_operations::{BitOps,MutBitProxy};

pub struct BitSlice<'a,ElementType,Pointer> {
    pub start_ptr: Pointer,
    pub end_ptr: *const ElementType,
    pub start_bit:u8,
    pub end_bit: u8,
    _life: PhantomData<&'a ElementType>
}

pub struct Biter<'a, ElementType,Pointer> {
    pub bit_slice: &'a BitSlice<'a,ElementType,Pointer>,
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

impl<'a, ElementType: BitSizes> Iterator for Biter<'a, ElementType,Immutable<ElementType> > {
    type Item = bool; // Yields true/false for individual bits

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pointer==self.bit_slice.end_ptr && self.current_bit==self.bit_slice.end_bit {return None}
        let bit = unsafe{(*self.current_pointer).get_bit(self.current_bit as usize)};
        self.current_bit += 1;
        self.current_pointer = unsafe {self.current_pointer.add(self.current_bit as usize>>ElementType::BIT_BITS)};
        self.current_bit&= ((ElementType::TYPE_BITS)-1) as u8; 
        Some(bit)
    }
}


impl <'a,ElementType:BitOps> BitSlice<'a,ElementType,Immutable<ElementType>> {
    pub fn type_idx(bitdex:usize) -> usize {bitdex/(ElementType::TYPE_BITS)}
    pub fn type_bit(bitdex:usize) -> u8 {(bitdex%(ElementType::TYPE_BITS)) as u8}

    pub fn iter(&'a self) -> Biter<'a, ElementType,Immutable<ElementType>> {
        Biter {
            bit_slice: &self,
            current_bit: self.start_bit,
            current_pointer: self.start_ptr
        }
    }

    pub fn get(&self, bitdex:usize) -> bool {
        let (ptr,bit) = self.bitdex_to_valid_ptr_bit(bitdex);
        unsafe { (*ptr).get_bit(bit as usize) }
    }

    pub unsafe fn bitdex_to_ptr_bit(&self, bitdex:usize) -> (Immutable<ElementType>,u8) {
        (unsafe { self.start_ptr.add(Self::type_idx(bitdex)) },Self::type_bit(bitdex))
    }

    pub fn bitdex_to_valid_ptr_bit(&self, bitdex:usize) -> (Immutable<ElementType>,u8) {
        let (ptr,bit) = unsafe { self.bitdex_to_ptr_bit(bitdex) };
        self.ptr_bit_bounds(ptr,bit);
        (ptr,bit)
    }

    pub fn ptr_bit_bounds(&self, ptr:Immutable<ElementType>, bit:u8) {
        if ptr<self.start_ptr || ptr>self.end_ptr {panic!("Pointer OutOfBounds")}
        else if ptr==self.start_ptr && bit<self.start_bit {panic!("Bit Lower OutOfBounds")}
        else if ptr == self.end_ptr && bit>self.end_bit {panic!("Bit Upper OutOfBounds")}
    }
}




impl <'a,ElementType:BitOps> BitSlice<'a,ElementType,Mutable<ElementType>> {
    pub fn set(&mut self, bitdex:usize, val:bool) {
        let (ptr,bit) = self.bitdex_to_valid_ptr_bit(bitdex);
        unsafe { (*ptr).set_bit(bit as usize, val) }
    }
}

// pub trait BitCollection<Type> {}
// impl <Type:BitOps>BitCollection<Type> for Vec<Type> {}
// impl <Type:BitOps,const N:usize>BitCollection<Type> for [Type;N] {}



// pub trait BitCollectionOps<'_,ElementType> {
//     fn biter() -> BitSlice<'_,ElementType,Immutable>;
//     fn biter_mut() -> BitSlice<'_,ElementType,Mutable>;
//     fn get(bitdex:usize) -> bool;
//     fn set(bitdex:usize,val:bool);
//     //pub fn get_mut(bitdex:usize)-> MutBitProxy<'_,ElementType>;
// }

//impl <ElementType:BitOps>BitCollectionOps<ElementType> for Vec<ElementType> {}



