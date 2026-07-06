# SliceBits (aka BitSlices)
turn any slice into a slice of bits (without mutation, consumption or cloning)
a wrapper around slices

Diffrent form version 1.0.0, everything now is in one unified struct BitSlice

## Examples

Get A bitslice
```rust
    use slicebits::BitSlice;
    let mut array: [u8;2] = [1,2];
    let immutable_bitslice = BitSlice::new(&array); //Immutable BitSlice
    let mutable_bitslice = BitSlice::new(&mut array); //Mutable BitSlice
```

Get And Set bits in a BitSlice
```rust
    use slicebits::BitSlice;
    let mut array: [u8;2] = [1,2];
    let mut mutable_bitslice = BitSlice::new(&mut array); //Mutable BitSlice
    assert_eq!(mutable_bitslice.get(0),true); //Immutable and Mutable method
    mutable_bitslice.set(0,false); //Mutable Method
    assert_eq!(mutable_bitslice.get(0),false);
    assert_eq!(array[0],0);
```

Get A mutable refrence to A bit in a Bitslice (proxy struct). MUST BE DROPPED FOR BIT TO UPDATE. DROP UPDATES!!!!
```rust
    use slicebits::BitSlice;
    let mut array: [u8;2] = [1,2];
    let mut mutable_bitslice = BitSlice::new(&mut array); //Mutable BitSlice
    {
        let mut bit = mutable_bitslice.get_mut(0); //Mutable ref to bit via Deref on proxy struct, updates bit on drop
        assert_eq!(*bit,true);
        *bit = false;
        assert_eq!(*bit,false);
    } //DROP UPDATES BIT
    assert_eq!(mutable_bitslice.get(0),false);
    assert_eq!(array[0],0);
```

Iterate over bits in a Bitslice
```rust
    use slicebits::BitSlice;
    let array: [u8;2] = [1,2];
    let bitslice = BitSlice::new(&array);
    let mut set_bits=0;
    bitslice.iter().for_each(|bit| set_bits+=bit as usize); //Imutable bools as return, implemented for Immutable and Mutable Bitslices
    assert_eq!(set_bits,2);

    let mut array = array;
    set_bits=0;
    let mut mbitslice = BitSlice::new(&mut array);
    mbitslice.iter_mut().for_each(|mut bit| *bit = true); //Mutable proxy struct (MutBitProxy) as return, implemented for Mutable Bitslices
    mbitslice.iter().for_each(|bit| set_bits+=bit as usize);
    assert_eq!(set_bits,2*8);
    assert_eq!(bitslice.len(),2*8); //Number of bits in BitSlice, Impl for Mutable and Immutable BitSlices
```

for full docs use docs.rs : [docs](https://docs.rs/slicebits/latest/slicebits/)


