pub trait ArabicChar {
    fn is_arabic_digit(&self) -> bool;
    #[allow(clippy::wrong_self_convention)]
    fn is_arabic_alphabetic(self) -> bool;

    /// (a)rabic (c)har (to) (e)nglish (c)har
    fn actoec(self) -> Option<char>;
    /// (e)nglish (c)har (to) (a)rabic (c)har
    fn ectoac(self) -> Option<char>;
}

impl ArabicChar for char {
    fn is_arabic_digit(&self) -> bool {
        ('\u{0660}'..='\u{0669}').contains(self)
    }

    fn is_arabic_alphabetic(self) -> bool {
        ('\u{0621}'..='\u{063A}').contains(&self) || ('\u{0641}'..='\u{064A}').contains(&self)
    }

    fn actoec(self) -> Option<char> {
        if self.is_arabic_digit() {
            return char::from_u32((self as u32) - ('\u{0660}' as u32) + ('0' as u32));
        }
        match self {
            // Arabic decimal seperator
            '\u{066B}' => Some('.'),
            _ => None,
        }
    }

    fn ectoac(self) -> Option<char> {
        if self.is_ascii_digit() {
            return char::from_u32((self as u32) - ('0' as u32) + ('\u{0660}' as u32));
        }
        if self == '.' {
            return Some('\u{066B}');
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::arabic_char::ArabicChar;

    #[test]
    fn it_should_return_true_when_provided_with_arabic_digit() {
        let c = '٢';
        assert_eq!(c.is_arabic_digit(), true);

        let c = '٣';
        assert_eq!(c.is_arabic_digit(), true);
    }

    #[test]
    fn it_should_transform_arabic_digit_into_english_digit() {
        let c = '٢';
        assert_eq!(c.actoec().unwrap(), '2');

        let c = '٣';
        assert_eq!(c.actoec().unwrap(), '3');
    }
}
