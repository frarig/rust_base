use std::io::{Read, Write, stdin, stdout};

fn main() {
    let (lines, words, bytes) = find_lines_words_bytes(stdin());

    print(lines, words, bytes);
}

fn find_lines_words_bytes<T: Read>(input: T) -> (u64, u64, u64) {
    let mut lines = 0u64;
    let mut words = 0u64;
    let mut bytes = 0u64;
    let mut in_word = false;

    input.bytes().for_each(|b| {
        let b = b.expect("Failed to read input");

        bytes += 1;

        if b == b'\n' {
            lines += 1;
        }

        let whitespace = b.is_ascii_whitespace();

        if !whitespace && !in_word {
            words += 1;
        }

        in_word = !whitespace;
    });

    if lines == 0 && words > 0 {
        lines += 1;
    }

    (lines, words, bytes)
}

fn print(lines: u64, words: u64, bytes: u64) {
    stdout()
        .write_all(format!("{lines} {words} {bytes}\n").as_bytes())
        .expect("Failed to write to stdout");
}

#[test]
fn test_find_lines_words_bytes() {
    assert_eq!(
        find_lines_words_bytes(&b""[..]),
        (0, 0, 0)
    );

    assert_eq!(
        find_lines_words_bytes(&b"  hi  \n"[..]),
        (1, 1, 7)
    );

    assert_eq!(
        find_lines_words_bytes(&b"foo"[..]),
        (1, 1, 3)
    );

    assert_eq!(
        find_lines_words_bytes(&b"foo\n"[..]),
        (1, 1, 4)
    );

    assert_eq!(
        find_lines_words_bytes(&b"Hello, World"[..]),
        (1, 2, 12)
    );

    assert_eq!(
        find_lines_words_bytes(&b"Hello, World\n"[..]),
        (1, 2, 13)
    );

    assert_eq!(
        find_lines_words_bytes(&b"y\te\ts"[..]),
        (1, 3, 5)
    );

    assert_eq!(
        find_lines_words_bytes(&b"y\te\ts\nyes\ny\te\ts\t\n"[..]),
        (3, 7, 17)
    );

    assert_eq!(
        find_lines_words_bytes(&b"y\te\ts\n y e s \ny\te\ts\t\n hello "[..]),
        (3, 10, 28)
    );
}
