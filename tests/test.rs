extern crate pod;
#[cfg(feature = "nue-codec")]
extern crate nue_codec;

use pod::{Pod, Le, Be};
use pod::packed::{Packed, Aligned, Un};

#[cfg(not(feature = "packed/oibit"))]
mod stable {
    use pod::packed::Unaligned;
    unsafe impl Unaligned for super::POD { }
}

unsafe impl Packed for POD { }
unsafe impl Pod for POD { }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
struct POD {
    zero: u8,
    ffff: Un<u16>,
    one: Le<u16>,
    two: Be<u32>,
}

const POD_BYTES: [u8; 9] = [
    0x00,
    0xff, 0xff,
    0x01, 0x00,
    0x00, 0x00, 0x00, 0x02,
];

fn sample() -> POD {
    POD {
        zero: 0,
        ffff: 0xffffu16.into_unaligned(),
        one: Le::new(1),
        two: Be::new(2),
    }
}

#[test]
#[cfg(feature = "nue-codec")]
fn pod_encoding() {
    use pod::Codable;
    use nue_codec::{Encode, Decode};
    use std::io::{Cursor, Seek, SeekFrom};

    let pod = sample();

    let buffer = Vec::new();
    let mut buffer = Cursor::new(buffer);

    Codable::new(pod).encode(&mut buffer).unwrap();

    buffer.seek(SeekFrom::Start(0)).unwrap();
    assert_eq!(pod, Codable::<POD>::decode(&mut buffer).unwrap().into_inner());

    let buffer = buffer.into_inner();

    assert_eq!(&buffer[..], &POD_BYTES[..]);
}

#[test]
fn pod_slice() {
    assert_eq!(sample(), *POD::from_bytes_ref(&POD_BYTES).unwrap())
}

#[test]
fn pod_box() {
    use std::iter::FromIterator;

    let vec: Vec<u8> = Vec::from_iter(POD_BYTES.iter().cloned());
    let boxed = POD::from_vec(vec).unwrap();
    assert_eq!(*boxed, sample());
}
