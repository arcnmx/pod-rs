extern crate nue_codec;

use std::io::{self, Read, Write};
use std::ops::{Deref, DerefMut};
use self::nue_codec::{Encode, Decode};
use Pod;

/// A wrapper around a POD type that implements `nue_codec::{Encode,Decode}` for
/// its plain memory representation.
pub struct Codable<T>(T);

impl<T> Deref for Codable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Codable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Codable<T> {
    /// Creates a new `Codable` containing the provided inner value.
    pub fn new(t: T) -> Self {
        Codable(t)
    }

    /// Moves the inner value out of `self`
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Pod> Encode for Codable<T> {
    type Options = ();

    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(self.as_bytes())
    }
}

impl<T: Pod> Decode for Codable<T> {
    type Options = ();

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        // TODO: Would be nice if we could use [0u8; size_of::<T>()]
        let mut pod: T = Pod::zeroed();

        try!(r.read_exact(pod.as_mut_bytes()));
        Ok(Codable(pod))
    }
}
