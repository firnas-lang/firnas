use crate::arabic_char::ArabicChar;
use crate::arabic_consts;

pub trait ArabicStr {
    fn parse_arabic_decimal(&self) -> Option<String>;
    fn is_arabic_number(&self) -> bool;
}

impl ArabicStr for str {
    fn is_arabic_number(&self) -> bool {
        if self.starts_with(arabic_consts::DECIMAL_SEPARATOR)
            || self.ends_with(arabic_consts::DECIMAL_SEPARATOR)
            || self.is_empty()
        {
            return false;
        }

        let mut sep_encountered = false;
        let mut res = true;
        self.chars().for_each(|c| {
            if !c.is_arabic_digit() && c != arabic_consts::DECIMAL_SEPARATOR {
                res = false;
            } else if c == arabic_consts::DECIMAL_SEPARATOR {
                if sep_encountered {
                    res = false;
                }
                sep_encountered = true
            }
        });

        res
    }

    fn parse_arabic_decimal(&self) -> Option<String> {
        if !self.is_arabic_number() {
            return None;
        }
        let s = self
            .chars()
            .map(|c| c.actoec().unwrap())
            .collect::<String>();
        Some(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::arabic_str::ArabicStr;

    #[test]
    fn it_should_not_be_arabic_number() {
        let s: &str = "";
        assert_eq!(s.is_arabic_number(), false);

        let s = "٫٣";
        assert_eq!(s.is_arabic_number(), false);

        let s = "٣٫";
        assert_eq!(s.is_arabic_number(), false);

        let s = "٣٫٣٫";
        assert_eq!(s.is_arabic_number(), false);

        let s = "٣٫٫٣";
        assert_eq!(s.is_arabic_number(), false);
    }

    #[test]
    fn it_should_be_arabic_number() {
        let s = "٣";
        assert_eq!(s.is_arabic_number(), true);

        let s = "٣٣";
        assert_eq!(s.is_arabic_number(), true);

        let s = "٣٫٣";
        assert_eq!(s.is_arabic_number(), true);
    }

    #[test]
    fn it_should_transform_arabic_number_to_english_number() {
        let s = "٣";
        assert_eq!(s.parse_arabic_decimal().unwrap(), "3");

        let s = "٣٣";
        assert_eq!(s.parse_arabic_decimal().unwrap(), "33");

        let s = "٣٫٣";
        assert_eq!(s.parse_arabic_decimal().unwrap(), "3.3");
    }
}
