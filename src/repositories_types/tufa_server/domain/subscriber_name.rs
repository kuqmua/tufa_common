#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn inner(self) -> String {
        self.0
    }
    pub fn inner_mut(&mut self) -> &mut str {
        &mut self.0
    }
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        // A grapheme is defined by the Unicode standard as a "user-perceived"
        // character: `å` is a single grapheme, but it is composed of two characters
        // (`a` and `̊`).
        //
        // `graphemes` returns an iterator over the graphemes in the input `s`.
        // `true` specifies that we want to use the extended grapheme definition set,
        // the recommended one.
        let is_too_long = {
            use unicode_segmentation::UnicodeSegmentation;
            s.graphemes(true)
        }.count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};

    #[test]
    fn unit_a_256_grapheme_long_name_is_valid() {
        let name = "a̐".repeat(256);
        assert_ok!(crate::domain::SubscriberName::parse(name));
    }

    #[test]
    fn unit_a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(crate::domain::SubscriberName::parse(name));
    }

    #[test]
    fn unit_whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(crate::domain::SubscriberName::parse(name));
    }

    #[test]
    fn unit_empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(crate::domain::SubscriberName::parse(name));
    }

    #[test]
    fn unit_names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(crate::domain::SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert_ok!(crate::domain::SubscriberName::parse(name));
    }
}
