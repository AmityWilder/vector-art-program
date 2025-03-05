use std::{io::{self, Read, Write}, mem::MaybeUninit};

pub trait AsBytes {
    type Buffer;

    fn buffer_ref(bytes: &Self::Buffer) -> &[u8];
    fn buffer_mut(bytes: &mut Self::Buffer) -> &mut [u8];

    fn from_le_bytes(bytes: Self::Buffer) -> Self;
    fn from_be_bytes(bytes: Self::Buffer) -> Self;
    fn from_ne_bytes(bytes: Self::Buffer) -> Self;

    fn to_le_bytes(self) -> Self::Buffer;
    fn to_be_bytes(self) -> Self::Buffer;
    fn to_ne_bytes(self) -> Self::Buffer;
}

macro_rules! impl_num_as_bytes {
    ($($T:ty),+ $(,)?) => {
        $(impl AsBytes for $T {
            type Buffer = [u8; {size_of::<$T>()}];

            #[inline]
            fn buffer_ref(bytes: &Self::Buffer) -> &[u8] {
                bytes
            }

            #[inline]
            fn buffer_mut(bytes: &mut Self::Buffer) -> &mut [u8] {
                bytes
            }

            #[inline]
            fn from_le_bytes(bytes: Self::Buffer) -> Self {
                <$T>::from_le_bytes(bytes)
            }

            #[inline]
            fn from_be_bytes(bytes: Self::Buffer) -> Self {
                <$T>::from_be_bytes(bytes)
            }

            #[inline]
            fn from_ne_bytes(bytes: Self::Buffer) -> Self {
                <$T>::from_ne_bytes(bytes)
            }

            #[inline]
            fn to_le_bytes(self) -> Self::Buffer {
                self.to_le_bytes()
            }

            #[inline]
            fn to_be_bytes(self) -> Self::Buffer {
                self.to_be_bytes()
            }

            #[inline]
            fn to_ne_bytes(self) -> Self::Buffer {
                self.to_ne_bytes()
            }
        })+
    };
}

impl_num_as_bytes! {
    u8,
    u16,
    u32,
    u64,
    u128,
    i8,
    i16,
    i32,
    i64,
    i128,
    f32,
    f64,
}

impl AsBytes for usize {
    type Buffer = [u8; size_of::<u64>()];

    #[inline]
    fn buffer_ref(bytes: &Self::Buffer) -> &[u8] {
        bytes
    }

    #[inline]
    fn buffer_mut(bytes: &mut Self::Buffer) -> &mut [u8] {
        bytes
    }

    #[inline]
    fn from_le_bytes(bytes: Self::Buffer) -> Self {
        <u64>::from_le_bytes(bytes) as usize
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Buffer) -> Self {
        <u64>::from_be_bytes(bytes) as usize
    }

    #[inline]
    fn from_ne_bytes(bytes: Self::Buffer) -> Self {
        <u64>::from_ne_bytes(bytes) as usize
    }

    #[inline]
    fn to_le_bytes(self) -> Self::Buffer {
        (self as u64).to_le_bytes()
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Buffer {
        (self as u64).to_be_bytes()
    }

    #[inline]
    fn to_ne_bytes(self) -> Self::Buffer {
        (self as u64).to_ne_bytes()
    }
}

impl AsBytes for bool {
    type Buffer = [u8; size_of::<u8>()];

    #[inline]
    fn buffer_ref(bytes: &Self::Buffer) -> &[u8] {
        bytes
    }

    #[inline]
    fn buffer_mut(bytes: &mut Self::Buffer) -> &mut [u8] {
        bytes
    }

    #[inline]
    fn from_le_bytes(bytes: Self::Buffer) -> Self {
        <u8>::from_le_bytes(bytes) != 0
    }

    #[inline]
    fn from_be_bytes(bytes: Self::Buffer) -> Self {
        <u8>::from_be_bytes(bytes) != 0
    }

    #[inline]
    fn from_ne_bytes(bytes: Self::Buffer) -> Self {
        <u8>::from_ne_bytes(bytes) != 0
    }

    #[inline]
    fn to_le_bytes(self) -> Self::Buffer {
        (self as u8).to_le_bytes()
    }

    #[inline]
    fn to_be_bytes(self) -> Self::Buffer {
        (self as u8).to_be_bytes()
    }

    #[inline]
    fn to_ne_bytes(self) -> Self::Buffer {
        (self as u8).to_ne_bytes()
    }
}

#[allow(clippy::uninit_assumed_init)]
pub trait ReadBytes: Read {
    /// Special cases:
    /// - [`usize`] reads as [`u64`] and gets converted using `as`
    /// - [`bool`] reads as [`u8`] and gets converted using `!= 0`
    fn read_le<T: AsBytes>(&mut self) -> io::Result<T> {
        let mut buf: T::Buffer = unsafe { MaybeUninit::uninit().assume_init() };
        self.read_exact(T::buffer_mut(&mut buf))?;
        Ok(T::from_le_bytes(buf))
    }

    /// Special cases:
    /// - [`usize`] reads as [`u64`] and gets converted using `as`
    /// - [`bool`] reads as [`u8`] and gets converted using `!= 0`
    fn read_be<T: AsBytes>(&mut self) -> io::Result<T> {
        let mut buf: T::Buffer = unsafe { MaybeUninit::uninit().assume_init() };
        self.read_exact(T::buffer_mut(&mut buf))?;
        Ok(T::from_be_bytes(buf))
    }

    /// Special cases:
    /// - [`usize`] reads as [`u64`] and gets converted using `as`
    /// - [`bool`] reads as [`u8`] and gets converted using `!= 0`
    fn read_ne<T: AsBytes>(&mut self) -> io::Result<T> {
        let mut buf: T::Buffer = unsafe { MaybeUninit::uninit().assume_init() };
        self.read_exact(T::buffer_mut(&mut buf))?;
        Ok(T::from_ne_bytes(buf))
    }
}

impl<R: Read> ReadBytes for R {}

pub trait WriteBytes: Write {
    /// Special cases:
    /// - [`usize`] reads as [`u64`] and gets converted using `as`
    /// - [`bool`] reads as [`u8`] and gets converted using `as`
    fn write_le<T: AsBytes>(&mut self, value: T) -> io::Result<()> {
        self.write_all(T::buffer_ref(&value.to_le_bytes()))
    }

    /// Special cases:
    /// - [`usize`] reads as [`u64`] and gets converted using `as`
    /// - [`bool`] reads as [`u8`] and gets converted using `as`
    fn write_be<T: AsBytes>(&mut self, value: T) -> io::Result<()> {
        self.write_all(T::buffer_ref(&value.to_be_bytes()))
    }

    /// Special cases:
    /// - [`usize`] reads as [`u64`] and gets converted using `as`
    /// - [`bool`] reads as [`u8`] and gets converted using `as`
    fn write_ne<T: AsBytes>(&mut self, value: T) -> io::Result<()> {
        self.write_all(T::buffer_ref(&value.to_ne_bytes()))
    }
}

impl<W: Write> WriteBytes for W {}

#[allow(clippy::uninit_assumed_init)]
pub trait ReadArr: ReadBytes {
    fn read_le_arr<T: AsBytes, const N: usize>(&mut self) -> io::Result<[T; N]> {
        let mut output = std::array::from_fn(|_| MaybeUninit::uninit());
        for item in &mut output {
            item.write(self.read_le()?);
        }
        Ok(unsafe { MaybeUninit::array_assume_init(output) })
    }

    fn read_be_arr<T: AsBytes, const N: usize>(&mut self) -> io::Result<[T; N]> {
        let mut output = std::array::from_fn(|_| MaybeUninit::uninit());
        for item in &mut output {
            item.write(self.read_be()?);
        }
        Ok(unsafe { MaybeUninit::array_assume_init(output) })
    }

    fn read_ne_arr<T: AsBytes, const N: usize>(&mut self) -> io::Result<[T; N]> {
        let mut output = std::array::from_fn(|_| MaybeUninit::uninit());
        for item in &mut output {
            item.write(self.read_ne()?);
        }
        Ok(unsafe { MaybeUninit::array_assume_init(output) })
    }
}

impl<R: ReadBytes> ReadArr for R {}

pub trait WriteArr: WriteBytes {
    fn write_le_arr<T: AsBytes, const N: usize>(&mut self, value: [T; N]) -> io::Result<()> {
        for item in value {
            self.write_le(item)?;
        }
        Ok(())
    }

    fn write_be_arr<T: AsBytes, const N: usize>(&mut self, value: [T; N]) -> io::Result<()> {
        for item in value {
            self.write_be(item)?;
        }
        Ok(())
    }

    fn write_ne_arr<T: AsBytes, const N: usize>(&mut self, value: [T; N]) -> io::Result<()> {
        for item in value {
            self.write_ne(item)?;
        }
        Ok(())
    }
}

impl<W: WriteBytes> WriteArr for W {}
