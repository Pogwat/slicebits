use bit_operations::{BitOps,MutBitProxy};
use core::marker::PhantomData;
struct BitSlice<'a,ElementType,S> {
    slice:S,
    sb:u8,
    eb:u8,
    _life: PhantomData<&'a ElementType>
}

impl <'a,ElementType:BitOps,S:AsRef<[ElementType]>> BitSlice<'a,ElementType,S> {
    pub fn bounds(&self,bit:usize) {if bit>=self.len() {panic!("Bit: {} is out of bounds as its greater than len: {}",bit,self.len())}}
    pub fn len(&self) -> usize {(self.slice.as_ref().len()-1)*ElementType::TYPE_BITS+(self.eb-self.sb) as usize}
    pub fn bits_idx(bit:usize) -> usize {bit/ElementType::TYPE_BITS}
    pub fn bits_bit(bit:usize) -> usize {bit%ElementType::TYPE_BITS}
    pub fn get(&self,bit:usize) -> bool {self.slice.as_ref()[Self::bits_idx(bit)].get_bit(Self::bits_bit(bit))}
    pub fn new(slice:S, sb:usize, eb:usize,) -> Self {Self {slice, sb:sb as u8, eb:eb as u8, _life: PhantomData}}
}

impl <'a,ElementType:BitOps,S:AsMut<[ElementType]>+AsRef<[ElementType]>> BitSlice<'a,ElementType,S> {
    pub fn set(&mut self,bit:usize, val:bool) {self.slice.as_mut()[Self::bits_idx(bit)].set_bit(Self::bits_bit(bit),val)}
    pub fn get_mut<'short>(&'short mut self, bit:usize) -> MutBitProxy<'short,ElementType> {self.slice.as_mut()[Self::bits_idx(bit)].mut_bit(Self::bits_bit(bit))}
}

// struct Biter<'short,'long,ElementType,S> {
//     bs: BitSlice<'long,ElementType,S>,
//     cb: usize,
//     _blife: PhantomData<&'short &'long ElementType>
// }

// struct BiterMut<'short,'long,ElementType,S> {
//     bs: BitSlice<'long,ElementType,S>,
//     cb: usize,
//     _blife: PhantomData<&'short &'long ElementType>
// }

// impl<'short, 'long, ElementType: BitOps, S:AsRef<[ElementType]> + AsMut<[ElementType]>> Iterator for BiterMut<'short, 'long, ElementType, S> {
//     type Item = MutBitProxy<'short, ElementType>;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.cb < self.bs.len() {
//             self.cb += 1;
//             // 2. Safely bypass the short &mut self constraint by casting the reference lifetime.
//             Some(unsafe {std::mem::transmute::<MutBitProxy<'_, ElementType>, MutBitProxy<'short, ElementType>>(self.bs.get_mut(self.cb-1)) })
//         } else {None}
//     }
// }

// impl <'short,'long,ElementType:BitOps,S:AsRef<[ElementType]>> Iterator for  Biter<'short,'long,ElementType,S> {
//     type Item=bool;
//     fn next(&mut self) -> Option<Self::Item> {
//     if self.cb<self.bs.len() {
//         self.cb += 1;
//         Some(self.bs.get(self.cb-1))
//     } else {None}}
// }

macro_rules! biterators {
    ( $( 
    (
        name: $name:ident, 
        (S: $($s_b:tt)*),
        ret: $ret:ty, 
        ret_code: |$self_var:ident| $ret_code:expr 
    ) 
    ),* ) => {
        $(
            struct $name<'short,'long,ElementType,S> {
                    bs: BitSlice<'long,ElementType,S>,
                    cb: usize,
                    _blife: PhantomData<&'short &'long ElementType>
            }
            impl<'short, 'long, ElementType: BitOps, S:$($s_b)* > Iterator for $name<'short, 'long, ElementType, S> {
                type Item = $ret;
                fn next(&mut self) -> Option<Self::Item> {
                    if self.cb < self.bs.len() {
                        self.cb += 1;
                        let $self_var = self;
                        $ret_code
                    } else {None}
                }
            }
        )*
    }
}

biterators!(
    (name: Biter, (S:AsRef<[ElementType]>),ret: bool, 
    ret_code: |this| Some(this.bs.get(this.cb - 1)) ),
     (name: BiterMut, (S:AsRef<[ElementType]>+AsMut<[ElementType]>),ret: MutBitProxy<'short, ElementType>, 
     ret_code: |this| Some(unsafe {std::mem::transmute::<MutBitProxy<'_, ElementType>, MutBitProxy<'short, ElementType>>(this.bs.get_mut(this.cb-1)) }) )

);
fn main() {
    println!("Hello, world!");
    let slice = &mut [1_u8,2_u8][0..=1];
    let mut bs = BitSlice::new(slice,0,0);
    assert_eq!(bs.get(0),true);
    {bs.get_mut(0);}
    assert_eq!(bs.get(1),false);
}
