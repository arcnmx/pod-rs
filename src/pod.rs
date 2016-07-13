use std::ptr::{copy_nonoverlapping, read};
use std::mem::{size_of, transmute, uninitialized, zeroed};
use std::slice::{from_raw_parts, from_raw_parts_mut};
use packed::{Unaligned, Aligned, is_aligned_for, is_aligned_for_slice, size_of_slice};

/// A marker trait indicating that a type is Plain Old Data.
///
/// It is unsafe to `impl` this manually, use `#[derive(Pod)]` instead.
pub unsafe trait Pod: Sized {
    /// Converts a POD reference from one to another type of the same size.
    ///
    /// Returns `None` if the two types are misaligned or not the same size.
    #[inline]
    fn map<T: Pod>(&self) -> Option<&T> {
        if size_of::<T>() == size_of::<Self>() && is_aligned_for::<T, _>(self) {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    /// Converts a mutable POD reference from one to another type of the same size.
    ///
    /// Returns `None` if the two types are misaligned or not the same size.
    #[inline]
    fn map_mut<T: Pod>(&mut self) -> Option<&mut T> {
        if size_of::<T>() == size_of::<Self>() && is_aligned_for::<T, _>(self) {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    /// Converts a POD type from one to another of the same size.
    ///
    /// Returns `None` if the two types are not the same size.
    #[inline]
    fn map_copy<T: Pod>(&self) -> Option<T> {
        if size_of::<T>() == size_of::<Self>() {
            Some(unsafe {
                Pod::copy_from_ptr(self.as_bytes().as_ptr())
            })
        } else {
            None
        }
    }

    /// Converts a POD reference from one to another type of the same or lesser size.
    ///
    /// Returns `None` if the two types are misaligned or `T` is larger.
    #[inline]
    fn try_map<T: Pod>(&self) -> Option<&T> {
        if size_of::<T>() <= size_of::<Self>() && is_aligned_for::<T, _>(self) {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    /// Converts a mutable POD reference from one to another type of the same or lesser size.
    ///
    /// Returns `None` if the two types are misaligned or `T` is larger.
    #[inline]
    fn try_map_mut<T: Pod>(&mut self) -> Option<&mut T> {
        if size_of::<T>() <= size_of::<Self>() && is_aligned_for::<T, _>(self) {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }

    /// Converts a POD type from one to another of the same or lesser size.
    ///
    /// Returns `None` if `T` is larger.
    #[inline]
    fn try_map_copy<T: Pod>(&self) -> Option<T> {
        if size_of::<T>() <= size_of::<Self>() {
            Some(unsafe {
                Pod::copy_from_ptr(self.as_bytes().as_ptr())
            })
        } else {
            None
        }
    }

    /// Converts a boxed POD type from one to another of the same size.
    ///
    /// Fails if the two types are misaligned or not the same size.
    #[inline]
    fn map_box<T: Pod>(self: Box<Self>) -> Result<Box<T>, Box<Self>> {
        if size_of::<T>() == size_of::<Self>() && is_aligned_for::<T, _>(&*self) {
            Ok(unsafe { Box::from_raw(Box::into_raw(self) as *mut _) })
        } else {
            Err(self)
        }
    }

    /// Converts a POD reference into a slice of another type.
    ///
    /// Returns `None` if the types are misaligned or do not fit perfectly.
    #[inline]
    fn split<T: Pod>(&self) -> Option<&[T]> {
        if size_of::<Self>() % size_of::<T>() == 0 && is_aligned_for::<T, _>(self) {
            Some(unsafe {
                from_raw_parts(self as *const _ as *const T, size_of::<Self>() / size_of::<T>())
            })
        } else {
            None
        }
    }

    /// Converts a mutable POD reference into a slice of another type.
    ///
    /// Returns `None` if the types are misaligned or do not fit perfectly.
    #[inline]
    fn split_mut<T: Pod>(&mut self) -> Option<&mut [T]> {
        if size_of::<Self>() % size_of::<T>() == 0 && is_aligned_for::<T, _>(self) {
            Some(unsafe {
                from_raw_parts_mut(self as *mut _ as *mut T, size_of::<Self>() / size_of::<T>())
            })
        } else {
            None
        }
    }

    /// Converts a POD reference into a slice of another type.
    ///
    /// Returns an empty slice if the types are misaligned.
    #[inline]
    fn try_split<T: Pod>(&self) -> &[T] {
        if is_aligned_for::<T, _>(self) {
            unsafe {
                from_raw_parts(self as *const _ as *const T, size_of::<Self>() / size_of::<T>())
            }
        } else {
            &[]
        }
    }

    /// Converts a mutable POD reference into a slice of another type.
    ///
    /// Returns an empty slice if the types are misaligned.
    #[inline]
    fn try_split_mut<T: Pod>(&mut self) -> &mut [T] {
        if is_aligned_for::<T, _>(self) {
            unsafe {
                from_raw_parts_mut(self as *mut _ as _, size_of::<Self>() / size_of::<T>())
            }
        } else {
            &mut []
        }
    }

    /// Converts a boxed POD object into a boxed slice of another type.
    ///
    /// Fails if the types are misaligned or do not fit perfectly.
    #[inline]
    fn split_box<T: Pod>(self: Box<Self>) -> Result<Box<[T]>, Box<Self>> {
        if size_of::<Self>() % size_of::<T>() == 0 && is_aligned_for::<T, _>(&*self) {
            Ok(unsafe {
                let ptr = Box::into_raw(self);
                Box::from_raw(
                    from_raw_parts_mut(ptr as _, size_of::<Self>() / size_of::<T>())
                )
            })
        } else {
            Err(self)
        }
    }

    /// Converts a boxed POD object into a vector of another type.
    ///
    /// Fails if the types are misaligned or do not fit perfectly.
    #[inline]
    fn split_vec<T: Pod>(self: Box<Self>) -> Result<Vec<T>, Box<Self>> {
        Self::split_box(self).map(|s| s.into_vec())
    }

    /// Maps a POD slice from one type to another.
    ///
    /// Returns `None` if the slice is misaligned or the output type does not perfectly fit.
    #[inline]
    fn map_slice<T: Pod>(s: &[Self]) -> Option<&[T]> {
        let len = size_of_slice(s);
        if is_aligned_for_slice::<T, _>(s) && len % size_of::<T>() == 0 {
            Some(unsafe {
                from_raw_parts(s.as_ptr() as _, len / size_of::<T>())
            })
        } else {
            None
        }
    }

    /// Maps a mutable POD slice from one type to another.
    ///
    /// Returns `None` if the slice is misaligned or the output type does not perfectly fit.
    #[inline]
    fn map_slice_mut<T: Pod>(s: &mut [Self]) -> Option<&mut [T]> {
        let len = size_of_slice(s);
        if is_aligned_for_slice::<T, _>(s) && len % size_of::<T>() == 0 {
            Some(unsafe {
                from_raw_parts_mut(s.as_mut_ptr() as _, len / size_of::<T>())
            })
        } else {
            None
        }
    }

    /// Maps a POD slice from one type to another.
    ///
    /// Returns `None` if the slice is misaligned.
    #[inline]
    fn try_map_slice<T: Pod>(s: &[Self]) -> &[T] {
        let len = size_of_slice(s);
        if is_aligned_for_slice::<T, _>(s) {
            unsafe {
                from_raw_parts(s.as_ptr() as _, len / size_of::<T>())
            }
        } else {
            &[]
        }
    }

    /// Maps a mutable POD slice from one type to another.
    ///
    /// Returns `None` if the slice is misaligned.
    #[inline]
    fn try_map_slice_mut<T: Pod>(s: &mut [Self]) -> &mut [T] {
        let len = size_of_slice(s);
        if is_aligned_for_slice::<T, _>(s) {
            unsafe {
                from_raw_parts_mut(s.as_mut_ptr() as _, len / size_of::<T>())
            }
        } else {
            &mut []
        }
    }

    /// Maps a boxed POD slice from one type to another.
    ///
    /// Fails if the slice is misaligned or does not perfectly fit.
    #[inline]
    fn map_slice_box<T: Pod>(s: Box<[Self]>) -> Result<Box<[T]>, Box<[Self]>> {
        let len = size_of_slice(&s);
        if is_aligned_for_slice::<T, _>(&s) && len % size_of::<T>() == 0 {
            Ok(unsafe {
                let ptr = Box::into_raw(s);
                Box::from_raw(
                    from_raw_parts_mut(ptr as _, len / size_of::<T>())
                )
            })
        } else {
            Err(s)
        }
    }

    /// Maps a POD vector from one type to another.
    ///
    /// Fails if the slice is misaligned or does not perfectly fit.
    #[inline]
    fn map_slice_vec<T: Pod>(s: Vec<Self>) -> Result<Vec<T>, Vec<Self>> {
        Self::map_slice_box(s.into_boxed_slice()).map_err(|s| s.into_vec()).map(|s| s.into_vec())
    }

    /// Converts a POD slice into another type.
    ///
    /// Returns `None` if the types are misaligned or not the same size.
    #[inline]
    fn merge<T: Pod>(s: &[Self]) -> Option<&T> {
        if is_aligned_for_slice::<T, _>(s) && size_of_slice(s) == size_of::<T>() {
            Some(unsafe { transmute(s.as_ptr()) })
        } else {
            None
        }
    }

    /// Converts a mutable POD slice into another type.
    ///
    /// Returns `None` if the types are misaligned or not the same size.
    #[inline]
    fn merge_mut<T: Pod>(s: &mut [Self]) -> Option<&mut T> {
        if is_aligned_for_slice::<T, _>(s) && size_of_slice(s) == size_of::<T>() {
            Some(unsafe { transmute(s.as_mut_ptr()) })
        } else {
            None
        }
    }

    /// Converts a POD slice into another type.
    ///
    /// Returns `None` if the types are not the same size.
    #[inline]
    fn merge_copy<T: Pod>(s: &[Self]) -> Option<T> {
        if size_of_slice(s) == size_of::<T>() {
            Some(unsafe {
                Pod::copy_from_ptr(s.as_ptr() as _)
            })
        } else {
            None
        }
    }

    /// Converts a POD slice into another type.
    ///
    /// Returns `None` if the types are misaligned or `T` is larger.
    #[inline]
    fn try_merge<T: Pod>(s: &[Self]) -> Option<&T> {
        if is_aligned_for_slice::<T, _>(s) && size_of_slice(s) >= size_of::<T>() {
            Some(unsafe { transmute(s.as_ptr()) })
        } else {
            None
        }
    }

    /// Converts a mutable POD slice into another type.
    ///
    /// Returns `None` if the types are misaligned or `T` is larger.
    #[inline]
    fn try_merge_mut<T: Pod>(s: &mut [Self]) -> Option<&mut T> {
        if is_aligned_for_slice::<T, _>(s) && size_of_slice(s) >= size_of::<T>() {
            Some(unsafe { transmute(s.as_mut_ptr()) })
        } else {
            None
        }
    }

    /// Converts a POD slice into another type.
    ///
    /// Returns `None` if `T` is larger.
    #[inline]
    fn try_merge_copy<T: Pod>(s: &[Self]) -> Option<T> {
        if size_of_slice(s) <= size_of::<T>() {
            Some(unsafe {
                Pod::copy_from_ptr(s.as_ptr() as _)
            })
        } else {
            None
        }
    }

    /// Converts a boxed POD slice into another boxed type.
    ///
    /// Fails if the types are misaligned or not the same size.
    #[inline]
    fn merge_box<T: Pod>(s: Box<[Self]>) -> Result<Box<T>, Box<[Self]>> {
        if is_aligned_for_slice::<T, _>(&s) && size_of_slice(&s) == size_of::<T>() {
            Ok(unsafe {
                let ptr = (*Box::into_raw(s)).as_mut_ptr();
                Box::from_raw(ptr as _)
            })
        } else {
            Err(s)
        }
    }

    /// Converts a POD vector into another boxed type.
    ///
    /// Fails if the types are misaligned or not the same size.
    #[inline]
    fn merge_vec<T: Pod>(s: Vec<Self>) -> Result<Box<T>, Vec<Self>> {
        Self::merge_box(s.into_boxed_slice()).map_err(|s| s.into_vec())
    }

    /// Creates a new POD instance with the inverse of `map_copy()`
    #[inline]
    fn from_ref<T: Pod>(p: &T) -> Option<Self> {
        Pod::map_copy(p)
    }

    /// Creates a new POD instance with the inverse of `merge_copy()`
    #[inline]
    fn from_slice<T: Pod>(p: &[T]) -> Option<Self> {
        Pod::merge_copy(p)
    }

    /// Creates a new POD instance with the inverse of `merge_box()`
    #[inline]
    fn from_boxed_slice<T: Pod>(p: Box<[T]>) -> Result<Box<Self>, Box<[T]>> {
        Pod::merge_box(p).or_else(|p|
            Pod::merge_copy(&p).map(Box::new).ok_or(p)
        )
    }

    /// Creates a new POD instance with the inverse of `merge_vec()`
    #[inline]
    fn from_vec<T: Pod>(p: Vec<T>) -> Result<Box<Self>, Vec<T>> {
        Pod::from_boxed_slice(p.into_boxed_slice()).map_err(|p| p.into_vec())
    }

    /// Creates a new POD instance with the inverse of `map_slice_box()`
    #[inline]
    fn slice_from_boxed_slice<T: Pod>(p: Box<[T]>) -> Result<Box<[Self]>, Box<[T]>> {
        Pod::map_slice_box(p)
    }

    /// Borrows the POD as a byte slice
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.try_split()
    }

    /// Borrows the POD as a mutable byte slice
    #[inline]
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.try_split_mut()
    }

    /// Safely creates a POD value from a potentially unaligned slice
    ///
    /// Returns `None` if `slice.len()` is not the same as the type's size
    #[inline]
    fn from_bytes(p: &[u8]) -> Option<Self> {
        Self::from_slice(&p)
    }

    /// Converts a boxed slice to a boxed instance of the POD type
    ///
    /// Fails if `slice.len()` is not the same as the type's size
    #[inline]
    fn from_byte_slice(p: Box<[u8]>) -> Result<Box<Self>, Box<[u8]>> {
        Self::from_boxed_slice(p)
    }

    /// Converts a byte vector to a boxed instance of the POD type
    ///
    /// Fails if `vec.len()` is not the same as the type's size
    #[inline]
    fn from_byte_vec(p: Vec<u8>) -> Result<Box<Self>, Vec<u8>> {
        Self::from_vec(p)
    }

    /// Converts a boxed POD to a boxed slice
    #[inline]
    fn into_byte_slice(self: Box<Self>) -> Box<[u8]> {
        Self::split_box(self).ok().unwrap()
    }

    /// Converts a boxed POD to a byte vector
    #[inline]
    fn into_byte_vec(self: Box<Self>) -> Vec<u8> {
        Self::split_vec(self).ok().unwrap()
    }

    #[doc(hidden)]
    fn __assert_pod() { }

    /// Safely borrows the aligned value mutably
    ///
    /// See also: `Aligned::from_unaligned_mut`
    #[inline]
    fn as_aligned_mut<T: Pod + Aligned<Unaligned=Self>>(&mut self) -> Option<&mut T> where Self: Copy + Unaligned {
        unsafe { Aligned::from_unaligned_mut(self) }
    }

    /// Safely borrows the unaligned value mutably
    ///
    /// See also: `Aligned::from_unaligned_mut`
    #[inline]
    fn from_unaligned_mut<T: Copy + Unaligned>(s: &mut T) -> Option<&mut Self> where Self: Aligned<Unaligned=T> {
        unsafe { Aligned::from_unaligned_mut(s) }
    }

    /// Safely converts an unaligned value to its aligned equivalent
    ///
    /// See also: `Aligned::from_unaligned`
    #[inline]
    fn from_unaligned<T: Copy + Unaligned>(s: T) -> Self where Self: Aligned<Unaligned=T> {
        unsafe { Aligned::from_unaligned(s) }
    }

    /*
    /// Borrows a new instance of the POD from a byte slice
    ///
    /// Returns `None` if `slice.len()` is not the same as the type's size
    #[inline]
    fn from_bytes_ref<'a>(slice: &'a [u8]) -> Option<&'a Self> where Self: Unaligned {
        if slice.len() == size_of::<Self>() {
            Some(unsafe { &*(slice.as_ptr() as *const _) })
        } else {
            None
        }
    }

    /// Borrows a mutable instance of the POD from a mutable byte slice
    ///
    /// Returns `None` if `slice.len()` is not the same as the type's size
    #[inline]
    fn from_bytes_mut<'a>(slice: &'a mut [u8]) -> Option<&'a mut Self> where Self: Unaligned {
        if slice.len() == size_of::<Self>() {
            Some(unsafe { &mut *(slice.as_mut_ptr() as *mut _) })
        } else {
            None
        }
    }
    */

    /// Generates a new uninitialized instance of a POD type.
    #[inline]
    unsafe fn uninitialized() -> Self {
        uninitialized()
    }

    /// Creates a new zeroed instance of a POD type.
    #[inline]
    fn zeroed() -> Self {
        unsafe { zeroed() }
    }

    /// Creates a copy of this POD instance
    #[inline]
    fn copy(&self) -> Self {
        unsafe {
            read(self)
        }
    }

    /// Creates a copy of this instance from an unaligned pointer
    #[inline]
    unsafe fn copy_from_ptr(source: *const u8) -> Self {
        let mut s = Self::uninitialized();
        copy_nonoverlapping(source, &mut s as *mut _ as _, size_of::<Self>());
        s
    }
}

unsafe impl Pod for () { }
unsafe impl Pod for f32 { }
unsafe impl Pod for f64 { }
unsafe impl Pod for i8 { }
unsafe impl Pod for u8 { }
unsafe impl Pod for i16 { }
unsafe impl Pod for u16 { }
unsafe impl Pod for i32 { }
unsafe impl Pod for u32 { }
unsafe impl Pod for i64 { }
unsafe impl Pod for u64 { }
unsafe impl Pod for isize { }
unsafe impl Pod for usize { }
unsafe impl<T> Pod for *const T { }
unsafe impl<T> Pod for *mut T { }

macro_rules! pod_def {
    ($($x:expr),*) => {
        $(
            unsafe impl<T: Pod> Pod for [T; $x] { }
        )*
    };
}

unsafe impl<T: Pod> Pod for (T,) { }
pod_def! {
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f,
    0x40,
    0x100, 0x200, 0x300, 0x400, 0x500, 0x600, 0x700, 0x800, 0x900, 0xa00, 0xb00, 0xc00, 0xd00, 0xe00, 0xf00,
    0x1000
}
