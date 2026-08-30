fn main() {
    println!("   checked: 10 + 30 = {:?}", add_u8_checked(10, 30));
    println!("  wrapping: 10 + 30 = {:?}", add_u8_wrapping(10, 30));
    println!("saturating: 10 + 30 = {:?}\n", add_u8_saturating(10, 30));
    println!("   checked: 255 + 1 = {:?}", add_u8_checked(255, 1));
    println!("  wrapping: 255 + 1 = {:?}", add_u8_wrapping(255, 1));
    println!("saturating: 255 + 1 = {:?}", add_u8_saturating(255, 1));
}

pub fn add_u8_checked(a: u8, b: u8) -> Option<u8> {
    if 255 - a < b {
        None
    } else {
        Some(a + b)
    }
}

pub fn add_u8_wrapping(a: u8, b: u8) -> u8 {
    let x = a as u16 + b as u16;
    x as u8
}

pub fn add_u8_saturating(a: u8, b: u8) -> u8 {
    if 255 - a < b {
        255
    } else {
        a + b
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn unsigned_overflow_modes() {
        assert_eq!(add_u8_checked(255, 1), None);
        assert_eq!(add_u8_wrapping(255, 1), 0);
        assert_eq!(add_u8_saturating(255, 1), 255);

        assert_eq!(add_u8_checked(123, 133), None);
        assert_eq!(add_u8_wrapping(123, 133), 0);
        assert_eq!(add_u8_saturating(123, 133), 255);

        assert_eq!(add_u8_checked(55, 200), Some(255));
        assert_eq!(add_u8_wrapping(55, 200), 255);
        assert_eq!(add_u8_saturating(55, 200), 255);

        assert_eq!(add_u8_checked(10, 20), Some(30));
        assert_eq!(add_u8_wrapping(10, 20), 30);
        assert_eq!(add_u8_saturating(10, 20), 30);

        assert_eq!(add_u8_checked(200, 255), None);
        assert_eq!(add_u8_wrapping(200, 255), 199);
        assert_eq!(add_u8_saturating(200, 255), 255);

        assert_eq!(add_u8_checked(255, 255), None);
        assert_eq!(add_u8_wrapping(255, 255), 254);
        assert_eq!(add_u8_saturating(255, 255), 255);
    }
}