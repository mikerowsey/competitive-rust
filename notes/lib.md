# I/O Modules — Scanner & Output

Zero-allocation I/O helpers: `Scanner` reads stdin into a buffer up front and `Output` accumulates results in memory, flushing once at end. Avoids per-call syscall overhead.

## Scanner

Reads all of stdin via `read_to_end()`, then parses tokens. All read methods skip leading ASCII whitespace.

| Method               | Returns   | Notes                                                                  |
| -------------------- | --------- | ---------------------------------------------------------------------- |
| `next_u64()`         | `u64`     | Unsigned 64-bit integer                                                |
| `next_i64()`         | `i64`     | Signed 64-bit; handles leading `-`; uses `wrapping_neg` for `i64::MIN` |
| `next_u32()`         | `u32`     | Unsigned 32-bit integer                                                |
| `next_i32()`         | `i32`     | Signed 32-bit integer                                                  |
| `next_usize()`       | `usize`   | Platform integer via `u64` cast                                        |
| `next_string()`      | `String`  | Whitespace-delimited token as UTF-8 String                             |
| `next_char()`        | `char`    | Single character after whitespace skip                                 |
| `next_line()`        | `Vec<u8>` | Bytes until newline (newline consumed)                                 |
| `Scanner::default()` | `Scanner` | Constructs from stdin                                                  |

## Output

Buffers output in `self.buf: Vec<u8>`. Nothing written to stdout until `flush()` or `print()` is called.

### Core Write Methods

| Method                   | Signature             | Description                        |
| ------------------------ | --------------------- | ---------------------------------- |
| `write<T: Writable>()`   | `&mut self, value: T` | Write any Writable type via trait  |
| `writeln<T: Writable>()` | `&mut self, value: T` | Write value + newline              |
| `write_byte()`           | `&mut self, b: u8`    | Append single byte                 |
| `endl()`                 | `&mut self`           | Append newline                     |
| `print()`                | `self`                | Write buffer to stdout and consume |

### Iterator Methods

| Method                              | Signature                       | Description                           |
| ----------------------------------- | ------------------------------- | ------------------------------------- |
| `write_iter<I, T>()`                | `&mut self, iter: I`            | Write items space-separated           |
| `write_iter_delimited<I, T>()`      | `&mut self, iter: I, delim: u8` | Write items with custom delimiter     |
| `writeln_iter<I, T>()`              | `&mut self, iter: I`            | Write items space-separated + newline |
| `write_slice<T: Writable + Copy>()` | `&mut self, slice: &[T]`        | Write slice space-separated           |

### Buffer Inspection

| Method       | Returns | Description              |
| ------------ | ------- | ------------------------ |
| `len()`      | `usize` | Current buffer size      |
| `capacity()` | `usize` | Allocated capacity       |
| `is_empty()` | `bool`  | True if no bytes written |

### Finalization

| Method              | Returns          | Description                           |
| ------------------- | ---------------- | ------------------------------------- |
| `flush(self)`       | `io::Result<()>` | Write buffer to stdout and consume    |
| `into_bytes(self)`  | `Vec<u8>`        | Return raw buffer without flushing    |
| `print(self)`       | `()`             | Write buffer to stdout via `unwrap()` |
| `Output::default()` | `Output`         | Constructs with 1 MiB pre-allocation  |

### Writable Trait

Implemented for: `bool`, `u8`–`u64`, `i8`–`i64`, `usize`, `isize`, `f32`, `f64`, `&str`, `String`, `&String`, `&[u8]`, `char`, and reference types (`&T` for all integer types and floats).

## Typical usage pattern

```rust
use cses::io::{Scanner, Output};

fn main() {
    let mut sc = Scanner::default();
    let mut out = Output::new();

    let n = sc.next_usize();
    let values: Vec<u64> = (0..n).map(|_| sc.next_u64()).collect();

    out.write_iter(&values);
    out.endl();
    out.flush().unwrap();
}
```
