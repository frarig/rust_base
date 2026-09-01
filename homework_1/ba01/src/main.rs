use std::io::{Read, Write, stdin, stdout};

fn main() {
    output(count_bytes(from_input(stdin())))
}

fn from_input<T: Read>(mut reader: T) -> usize {
    let mut res = 0usize;
    let mut r;

    loop {
        r = reader.read(&mut [0u8; 1024]).expect("Failed to read input");

        if r == 0 {
            break;
        }
        res += r;
    }

    res
}

fn count_bytes(input: usize) -> Vec<u8> {
    input.to_string().into_bytes()
}

fn output(res: Vec<u8>) {
    stdout().write_all(&res).expect("Failed to write to stdout");

    stdout()
        .write_all(b"\n")
        .expect("Failed to write to stdout");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_input() {
        assert_eq!(from_input(&b"hello"[..]), 5);
        assert_eq!(from_input(&b"Hello, World!"[..]), 13);
        assert_eq!(from_input(&b""[..]), 0);
        assert_eq!(from_input(&"🦀".as_bytes()[..]), 4);
        assert_eq!(from_input(&b"   "[..]), 3);
        assert_eq!(from_input(&b"Hello Hello Hello Hello Hello\n"[..]), 30);
    }

    #[test]
    fn test_count_bytes() {
        assert_eq!(count_bytes(5), [53]);
        assert_eq!(count_bytes(13), [49, 51]);
        assert_eq!(count_bytes(0), [48]);
        assert_eq!(count_bytes(4), [52]);
    }
}
