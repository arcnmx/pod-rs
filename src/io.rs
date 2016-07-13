use std::io;
use pod::Pod;

#[cfg(feature = "uninitialized")]
use uninitialized::uninitialized;
#[cfg(not(feature = "uninitialized"))]
use std::mem::zeroed as uninitialized;

#[cfg(feature = "read_exact")]
use read_exact::ReadExactExt;

/// An extension trait for reading `Pod` types from `std::io::Read` data streams.
pub trait PodReadExt {
    /// Reads a `Pod` struct from the stream. Behaves like `read_exact`, and will
    /// produce an error if EOF is encountered before the data is fully read.
    fn read_pod<P: Pod>(&mut self) -> io::Result<P>;

    /// Reads a `Pod` struct from the stream, or nothing at EOF. Partial reads
    /// will result in an error.
    #[cfg(feature = "read_exact")]
    fn read_pod_or_none<P: Pod>(&mut self) -> io::Result<Option<P>>;
}

impl<T: io::Read> PodReadExt for T {
    #[inline]
    fn read_pod<P: Pod>(&mut self) -> io::Result<P> {
        let mut data: P = unsafe { uninitialized() };

        self.read_exact(data.as_bytes_mut()).map(|_| data)
    }

    #[inline]
    #[cfg(feature = "read_exact")]
    fn read_pod_or_none<P: Pod>(&mut self) -> io::Result<Option<P>> {
        let mut data: P = unsafe { uninitialized() };

        self.read_exact_or_eof(data.as_bytes_mut()).map(|read| if read {
            Some(data)
        } else {
            None
        })
    }
}

/// An extension trait for writing `Pod` types to `std::io::Write` data streams.
pub trait PodWriteExt {
    /// Writes the memory representation of a `Pod` struct to the stream.
    /// Behaves like `write_all`, failure to write the entire structure will
    /// result in an error.
    fn write_pod<P: Pod>(&mut self, data: &P) -> io::Result<()>;
}

impl<T: io::Write> PodWriteExt for T {
    #[inline]
    fn write_pod<P: Pod>(&mut self, data: &P) -> io::Result<()> {
        self.write_all(data.as_bytes())
    }
}
