use std::{
    fmt,
    io::{self, Write},
};

#[must_use]
pub struct Output {
    buf: Vec<u8>,
}

pub trait Writable {
    fn write_to(self, out: &mut Output);
}

impl Output {
    pub fn new() -> Self {
        Self::with_capacity(1 << 20)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn write<T: Writable>(&mut self, value: T) {
        value.write_to(self);
    }

    #[inline]
    pub fn writeln<T: Writable>(&mut self, value: T) {
        self.write(value);
        self.endl();
    }

    #[inline]
    pub fn write_iter<I, T>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
        T: Writable,
    {
        let mut first = true;

        for value in iter {
            if !first {
                self.write_byte(b' ');
            }

            first = false;
            self.write(value);
        }
    }

    #[inline]
    pub fn write_iter_delimited<I, T>(&mut self, iter: I, delimiter: u8)
    where
        I: IntoIterator<Item = T>,
        T: Writable,
    {
        let mut first = true;

        for value in iter {
            if !first {
                self.write_byte(delimiter);
            }

            first = false;
            self.write(value);
        }
    }

    #[inline]
    pub fn writeln_iter<I, T>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
        T: Writable,
    {
        self.write_iter(iter);
        self.endl();
    }

    #[inline]
    pub fn write_slice<T: Writable + Copy>(&mut self, slice: &[T]) {
        self.write_iter(slice.iter().copied());
    }

    #[inline]
    pub fn endl(&mut self) {
        self.write_byte(b'\n');
    }

    pub fn print(self) {
        io::stdout().write_all(&self.buf).unwrap();
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn flush(self) -> io::Result<()> {
        io::stdout().write_all(&self.buf)
    }

    #[inline]
    fn write_bytes(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes);
    }

    #[inline]
    pub fn write_byte(&mut self, byte: u8) {
        self.buf.push(byte);
    }

    #[inline]
    fn write_u64(&mut self, mut value: u64) {
        if value == 0 {
            self.write_byte(b'0');
            return;
        }

        let start = self.buf.len();

        while value > 0 {
            self.write_byte(b'0' + (value % 10) as u8);
            value /= 10;
        }

        self.buf[start..].reverse();
    }

    #[inline]
    fn write_f64(&mut self, value: f64) {
        // Format float as string to avoid macro scope issues
        let s = value.to_string();
        self.write_bytes(s.as_bytes());
    }
}

impl fmt::Write for Output {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_bytes(s.as_bytes());
        Ok(())
    }
}

impl Writable for bool {
    fn write_to(self, out: &mut Output) {
        if self {
            out.write_bytes(b"true");
        } else {
            out.write_bytes(b"false");
        }
    }
}

impl Writable for u8 {
    fn write_to(self, out: &mut Output) {
        out.write_u64(self as u64);
    }
}

impl Writable for u16 {
    fn write_to(self, out: &mut Output) {
        out.write_u64(self as u64);
    }
}

impl Writable for u32 {
    fn write_to(self, out: &mut Output) {
        out.write_u64(self as u64);
    }
}

impl Writable for u64 {
    fn write_to(self, out: &mut Output) {
        out.write_u64(self);
    }
}

impl Writable for usize {
    fn write_to(self, out: &mut Output) {
        out.write_u64(self as u64);
    }
}

impl Writable for i8 {
    fn write_to(self, out: &mut Output) {
        (self as i64).write_to(out);
    }
}

impl Writable for i16 {
    fn write_to(self, out: &mut Output) {
        (self as i64).write_to(out);
    }
}

impl Writable for i32 {
    fn write_to(self, out: &mut Output) {
        (self as i64).write_to(out);
    }
}

impl Writable for i64 {
    fn write_to(self, out: &mut Output) {
        if self < 0 {
            out.write_byte(b'-');
            out.write_u64(self.unsigned_abs());
        } else {
            out.write_u64(self as u64);
        }
    }
}

impl Writable for isize {
    fn write_to(self, out: &mut Output) {
        (self as i64).write_to(out);
    }
}

impl Writable for f32 {
    fn write_to(self, out: &mut Output) {
        out.write_f64(self as f64);
    }
}

impl Writable for f64 {
    fn write_to(self, out: &mut Output) {
        out.write_f64(self);
    }
}

impl Writable for &f32 {
    #[inline]
    fn write_to(self, out: &mut Output) {
        out.write(*self);
    }
}

impl Writable for &f64 {
    #[inline]
    fn write_to(self, out: &mut Output) {
        out.write(*self);
    }
}

impl Writable for &str {
    fn write_to(self, out: &mut Output) {
        out.write_bytes(self.as_bytes());
    }
}

impl Writable for String {
    fn write_to(self, out: &mut Output) {
        out.write_bytes(self.as_bytes());
    }
}

impl Writable for &String {
    fn write_to(self, out: &mut Output) {
        out.write_bytes(self.as_bytes());
    }
}

impl Writable for &[u8] {
    fn write_to(self, out: &mut Output) {
        out.write_bytes(self);
    }
}

impl Writable for char {
    fn write_to(self, out: &mut Output) {
        let mut buf = [0; 4];
        out.write_bytes(self.encode_utf8(&mut buf).as_bytes());
    }
}

impl Writable for &u64 {
    #[inline]
    fn write_to(self, out: &mut Output) {
        out.write(*self);
    }
}

impl Writable for &u32 {
    #[inline]
    fn write_to(self, out: &mut Output) {
        out.write(*self);
    }
}

impl Writable for &u16 {
    #[inline]
    fn write_to(self, out: &mut Output) {
        out.write(*self);
    }
}

impl Writable for &u8 {
    #[inline]
    fn write_to(self, out: &mut Output) {
        out.write(*self);
    }
}

impl Writable for &i64 {
    #[inline]
    fn write_to(self, out: &mut Output) {
        out.write(*self);
    }
}

impl Writable for &i32 {
    #[inline]
    fn write_to(self, out: &mut Output) {
        out.write(*self);
    }
}

impl Writable for &i16 {
    #[inline]
    fn write_to(self, out: &mut Output) {
        out.write(*self);
    }
}

impl Writable for &i8 {
    #[inline]
    fn write_to(self, out: &mut Output) {
        out.write(*self);
    }
}

impl Writable for &usize {
    #[inline]
    fn write_to(self, out: &mut Output) {
        out.write(*self);
    }
}

impl Writable for &isize {
    #[inline]
    fn write_to(self, out: &mut Output) {
        out.write(*self);
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}
