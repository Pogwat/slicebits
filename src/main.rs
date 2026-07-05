use bit_operations::{BitOps,MutBitProxy};
use core::marker::PhantomData;
struct BitSlice<'a,ElementType,S> {
    slice:S,
    sb:u8,
    eb:u8,
    _life: PhantomData<&'a ElementType>
}

pub trait ImmutableBitSlice<ElementType:BitOps, S:AsRef<[ElementType]>>:Sized {
    fn bounds(&self,bit:usize); 
    fn len(&self) -> usize;
    fn bits_idx(bit:usize) -> usize;
    fn bits_bit(bit:usize) -> usize;
    fn get(&self,bit:usize) -> bool;
    fn new(slice:S, sb:usize, eb:usize) -> Self;
    fn iter<'short>(&'short self) -> Biter<'short,ElementType>;
}

pub trait MutableBitSlice<ElementType:BitOps, S:AsMut<[ElementType]>+AsRef<[ElementType]>> {
    fn set(&mut self,bit:usize, val:bool);
    fn get_mut<'short>(&'short mut self, bit:usize) -> MutBitProxy<'short,ElementType>;
    fn iter_mut<'short>(&'short mut self) -> MutBiter<'short, ElementType>;
}

impl <'a,ElementType:BitOps,S:AsRef<[ElementType]>> ImmutableBitSlice<ElementType,S> for BitSlice<'a,ElementType,S> {
    fn bounds(&self,bit:usize) {if bit>=self.len() {panic!("Bit: {} is out of bounds as its greater than len: {}",bit,self.len())}}
    fn len(&self) -> usize {(self.slice.as_ref().len()-1)*ElementType::TYPE_BITS+(self.eb-self.sb) as usize+1}
    fn bits_idx(bit:usize) -> usize {bit/ElementType::TYPE_BITS}
    fn bits_bit(bit:usize) -> usize {bit%ElementType::TYPE_BITS}
    fn get(&self,bit:usize) -> bool {self.slice.as_ref()[Self::bits_idx(bit)].get_bit(Self::bits_bit(bit))}
    fn new(slice:S, sb:usize, eb:usize,) -> Self {Self {slice, sb:sb as u8, eb:eb as u8, _life: PhantomData}}
    fn iter<'short>(&'short self) -> Biter<'short,ElementType> {Biter::from(self)}
}

impl <'a,ElementType:BitOps,S:AsMut<[ElementType]>+AsRef<[ElementType]>> MutableBitSlice<ElementType, S> for BitSlice<'a,ElementType,S> {
    fn set(&mut self,bit:usize, val:bool) {self.slice.as_mut()[Self::bits_idx(bit)].set_bit(Self::bits_bit(bit),val)}
    fn get_mut<'short>(&'short mut self, bit:usize) -> MutBitProxy<'short,ElementType> {self.slice.as_mut()[Self::bits_idx(bit)].mut_bit(Self::bits_bit(bit))}
    fn iter_mut<'short>(&'short mut self) -> MutBiter<'short,ElementType> {MutBiter::from(self)}
}

// macro_rules! biterators {
//     ( $( (name: $name:ident, (S: $($s_b:tt)*),ret: $ret:ty, ret_code: |$self_var:ident| $ret_code:expr) ),* ) => {$(
//         struct $name<'short,'long,ElementType,S> {
//                 bs: BitSlice<'long,ElementType,S>,
//                 cb: usize,
//                 _blife: PhantomData<&'short &'long ElementType>
//         }
//         impl<'short, 'long, ElementType: BitOps, S:$($s_b)* > Iterator for $name<'short, 'long, ElementType, S> {
//             type Item = $ret;
//             fn next(&mut self) -> Option<Self::Item> {
//                 if self.cb < self.bs.len() {
//                     self.cb += 1;
//                     let $self_var = self; $ret_code
//                 } else {None}
//             }
//         }

//         impl<'short, 'long, ElementType: BitOps, S:$($s_b)* > From<BitSlice<'long,ElementType,S>> for $name<'short,'long,ElementType,S>{
//             fn from(bs:BitSlice<'long,ElementType,S>) -> Self {Self {bs, cb:0, _blife: PhantomData} }
//         }
//     )*}}


// biterators!(
//     (name: Biter, (S:AsRef<[ElementType]>),ret: bool, 
//     ret_code: |this| Some(this.bs.get(this.cb - 1)) )
//     // ,(name: BiterMut, (S:AsRef<[ElementType]>+AsMut<[ElementType]>),ret: MutBitProxy<'short, ElementType>, 
//     // ret_code: |this| Some(unsafe {std::mem::transmute::<MutBitProxy<'_, ElementType>, MutBitProxy<'short, ElementType>>(this.bs.get_mut(this.cb-1)) }) )

// );

        // struct Biter<'short,'long,ElementType> {
        //         cp: *const ElementType,
        //         rb: usize,
        //         b:u8,
        //         _blife: PhantomData<&'short &'long ElementType>,
        // }
        // impl<'short, 'long, ElementType: BitOps> Iterator for Biter<'short, 'long, ElementType> {
        //     type Item = bool;
        //     fn next(&mut self) -> Option<Self::Item> {
        //         if self.rb!=0 {
        //             let bit = unsafe {(*self.cp).get_bit(self.b as usize) };
        //             self.b+=1;
        //             self.rb-=1;
        //             if self.b==ElementType::TYPE_BITS as u8 {
        //                 self.b=0;
        //                 unsafe {self.cp = self.cp.add(1)};
        //             }
        //             Some(bit)
        //         } else {None}
        //     }
        // }



        // struct MutBiter<'short,'long,ElementType> {
        //         cp: *mut ElementType,
        //         rb: usize,
        //         b:u8,
        //         _blife: PhantomData<&'short mut &'long ElementType>,
        // }
        // impl<'short, 'long, ElementType: BitOps> Iterator for MutBiter<'short, 'long, ElementType> {
        //     type Item = MutBitProxy<'short,ElementType>;
        //     fn next(&mut self) -> Option<Self::Item> {
        //         if self.rb!=0 {
        //             let bit = unsafe {(*self.cp).mut_bit(self.b as usize) };
        //             self.b+=1;
        //             self.rb-=1;
        //             if self.b==ElementType::TYPE_BITS as u8 {
        //                 self.b=0;
        //                 unsafe {self.cp = self.cp.add(1)};
        //             }
        //             Some(bit)
        //         } else {None}
        //     }
        // }

        macro_rules! biterators {
            (name:$name:ident, item:$item:ty, bit_method:$bit_method:ident, ptr_ty:$ptr_ty:tt  $(, lock:$lock:tt)? ) => {
                pub struct $name<'short,ElementType> {
                    cp: *$ptr_ty ElementType,
                    rb: usize,
                    b:u8,
                    _blife: PhantomData<&'short $($lock)? ElementType>,
                }
                impl<'short, ElementType: BitOps> Iterator for $name<'short, ElementType> {
                    type Item = $item;
                    fn next(&mut self) -> Option<Self::Item> {
                        if self.rb!=0 {
                            let bit = unsafe {(*self.cp).$bit_method(self.b as usize) };
                            self.b+=1;
                            self.rb-=1;
                            if self.b==ElementType::TYPE_BITS as u8 {
                                self.b=0;
                                unsafe {self.cp = self.cp.add(1)};
                            }
                            Some(bit)
                        } else {None}
                    }
                }
                impl<'short, ElementType: BitOps> $name<'short,ElementType>{
                    pub fn new(cp:*$ptr_ty ElementType, b:u8, rb:usize) -> Self {Self {cp, b, rb, _blife: PhantomData} }
                }

                impl <'short,ElementType: BitOps > From<&'short $($lock)? [ElementType]> for $name<'short,ElementType> {
                    fn from(s:&'short $($lock)? [ElementType]) -> Self {
                        Self::new(s as *$ptr_ty [ElementType] as *$ptr_ty ElementType,0,s.len()*ElementType::TYPE_BITS)
                    }
                }
            }
        }
        biterators!(name:Biter,item:bool,bit_method:get_bit, ptr_ty:const);
        biterators!(name:MutBiter,item:MutBitProxy<'short,ElementType>,bit_method:mut_bit, ptr_ty:mut, lock:mut);

        impl <'short,'long, ElementType: BitOps,S:AsRef<[ElementType]> > From<&'short BitSlice<'long,ElementType,S>> for Biter<'short,ElementType> {
            fn from(bs:&'short BitSlice<'long,ElementType,S>) -> Self {
                Self::new(bs.slice.as_ref() as *const [ElementType] as *const ElementType,bs.sb,bs.len())
            }
        }

        impl <'short, 'long, ElementType: BitOps,S:AsRef<[ElementType]> + AsMut<[ElementType]>> From<&'short mut BitSlice<'long,ElementType,S>> for MutBiter<'short,ElementType> {
            fn from(bs:&'short mut BitSlice<'long,ElementType,S>) -> Self {
                Self::new(bs.slice.as_mut() as *mut [ElementType] as *mut ElementType,bs.sb,bs.len())
            }
        }






fn main() {
    println!("Hello, world!");
    let slice = &mut [1_u8,u8::MAX][0..=1];
    let mut bs = BitSlice::new(slice,0,7);
    assert_eq!(bs.get(0),true);
    {bs.get_mut(0);}
    assert_eq!(bs.get(1),false);
    bs.iter_mut().for_each(|mut bit|{println!("{}",*bit);});
    bs.iter().for_each(|bit|println!("{}",bit));
    println!("{}",bs.len());
}
