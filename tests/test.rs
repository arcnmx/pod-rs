extern crate pod;

use pod::Pod;

#[repr(C)]
struct Unaligned {
    _pad0: u8,
    value: [u8; 2],
    _pad1: u8,
    _align: u32,
}

#[test]
fn test_map() {
    let mut un = Unaligned {
        _pad0: 0,
        value: [1, 1],
        _pad1: 0,
        _align: 0,
    };
    let un = &mut un.value;

    assert!(0u16.map::<u8>().is_none());
    assert!(0u16.map_mut::<u8>().is_none());
    assert!(0u16.map_copy::<u8>().is_none());
    assert!(0u16.try_map::<u8>().unwrap() == &0);
    assert!(0u16.try_map_mut::<u8>().unwrap() == &mut 0);
    assert!(Pod::map_box::<u8>(Box::new(0u16)).is_err());

    assert!(0u16.map::<u32>().is_none());
    assert!(0u16.map_mut::<u32>().is_none());
    assert!(0u16.map_copy::<u32>().is_none());
    assert!(0u16.try_map::<u32>().is_none());
    assert!(0u16.try_map_mut::<u32>().is_none());
    assert!(Pod::map_box::<u32>(Box::new(0u16)).is_err());

    assert!(0xffu16.map::<i16>().unwrap() == &0xffi16);
    assert!(0xffu16.map_mut::<i16>().unwrap() == &mut 0xffi16);
    assert!(0xffu16.map_copy::<i16>().unwrap() == 0xffi16);
    assert!(0xffu16.try_map::<i16>().unwrap() == &0xff);
    assert!(0xffu16.try_map_mut::<i16>().unwrap() == &mut 0xff);
    assert!(*Pod::map_box::<i16>(Box::new(0xffu16)).unwrap() == 0xff);

    assert!(un.map::<i16>().is_none());
    assert!(un.map_mut::<i16>().is_none());
    assert!(un.map_copy::<i16>().unwrap() == 0x0101i16);
    assert!(un.try_map::<i16>().is_none());
    assert!(un.try_map_mut::<i16>().is_none());
}
