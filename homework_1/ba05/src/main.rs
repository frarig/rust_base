fn main() {
    let image = [
        "..####..",
        ".#....#.",
        "#.#..#.#",
        "#..##..#",
        "#......#",
        "#.#..#.#",
        ".#....#.",
        "..####..",
    ];

    let bytes = parse_bitmap_8x8(image);

    println!("Bytes:");
    for byte in bytes {
        println!("{byte:08b}    0x{byte:02x}");
    }

    println!();
    println!("Rendered:");

    for line in render_bitmap_8x8(bytes) {
        println!("{line}");
    }

    println!();
    println!("Inverted:");

    for line in render_bitmap_8x8(invert_bitmap_8x8(bytes)) {
        println!("{line}");
    }
}

pub fn parse_bitmap_8x8(lines: [&str; 8]) -> [u8; 8] {
    let mut res = [0u8; 8];
    let mut bytes;
    let mut row;

    for i in 0..lines.len() {
        bytes = lines[i].as_bytes();
        row = 0;

        for j in 0..8 {
            if bytes[j] == b'#' {
                row |= 1 << (7 - j);
            }
        }

        res[i] = row;
    }

    res
}

pub fn render_bitmap_8x8(bytes: [u8; 8]) -> [String; 8] {
    let mut s: [String; 8] = Default::default();

    for i in 0..bytes.len() {
        s[i] = format!("{:08b}", bytes[i])
            .replace("0", ".")
            .replace("1", "#");
    }

    s
}

pub fn invert_bitmap_8x8(bytes: [u8; 8]) -> [u8; 8] {
    let mut res = [0u8; 8];

    for i in 0..bytes.len() {
        res[i] = !bytes[i];
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_bitmap_8x8() {
        let image = [
            "..####..",
            ".#....#.",
            "#.#..#.#",
            "#......#",
            "#.#..#.#",
            "#..##..#",
            ".#....#.",
            "..####..",
        ];

        let bytes = parse_bitmap_8x8(image);

        assert_eq!(bytes[1], 0b01000010);
        assert_eq!(bytes[3], 0b10000001);
        assert_eq!(bytes[5], 0b10011001);
        assert_eq!(bytes[7], 0b00111100);
    }

    #[test]
    fn test_render_bitmap_8x8() {
        let bytes = [
            0b00111100,
            0b01000010,
            0b10000001,
            0b10011001,
            0b10011001,
            0b10011001,
            0b10011001,
            0b10011001,
        ];

        let lines = render_bitmap_8x8(bytes);

        assert_eq!(lines[0], "..####..");
        assert_eq!(lines[2], "#......#");
        assert_eq!(lines[4], "#..##..#");
        assert_eq!(lines[6], "#..##..#");
    }

    #[test]
    fn test_invert_bitmap_8x8() {
        let bytes = [
            0b00111100,
            0b01000010,
            0b10000001,
            0b10011001,
            0b10100101,
            0b11000011,
            0b10100101,
            0b10011001,
        ];

        let res = invert_bitmap_8x8(bytes);

        assert_eq!(res[1], 0b10111101);
        assert_eq!(res[3], 0b01100110);
        assert_eq!(res[5], 0b00111100);
        assert_eq!(res[7], 0b01100110);
    }
}