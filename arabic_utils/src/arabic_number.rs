use std::f32::NAN;

use crate::arabic_char::ArabicChar;

pub trait ArabicNumber {
    fn to_arabic_decimal(&self) -> Option<String>;
}

impl ArabicNumber for f64 {
    fn to_arabic_decimal(&self) -> Option<String> {
        if self == &f64::INFINITY {
            return Some(String::from("لانهاية"));
        }
        if self == &f64::NEG_INFINITY {
            todo!("Handle negative infinity")
        }
        if self.is_nan() {
            todo!("Handle not a number (for example divide zero by itself)")
        }

        let num = format!("{self}")
            .chars()
            .map(|c| c.ectoac().unwrap())
            .collect::<String>();

        Some(num)
    }
}

#[cfg(test)]
mod tests {
    use super::ArabicNumber;

    #[test]
    fn it_should_transform_arabic_number_to_english_number() {
        let s = 3.0;
        assert_eq!(s.to_arabic_decimal().unwrap(), "٣");

        let s = 33.0;
        assert_eq!(s.to_arabic_decimal().unwrap(), "٣٣");

        let s = 3.3;
        assert_eq!(s.to_arabic_decimal().unwrap(), "٣٫٣");
    }
}
