#[derive(Debug)]
pub struct SubscriberEmail(String);

impl TryFrom<std::string::String> for SubscriberEmail {
    type Error = std::string::String;
    fn try_from(possible_email: std::string::String) -> Result<Self, std::string::String> {
        if validator::validate_email(&possible_email) {
            Ok(Self(possible_email))
        } else {
            Err(format!("{possible_email} is not a valid subscriber email."))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SubscriberEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::SubscriberEmail;
//     use claim::assert_err;
//     use fake::faker::internet::en::SafeEmail;
//     use fake::Fake;

//     #[test]
//     fn unit_empty_string_is_rejected() {
//         let email = "".to_string();
//         assert_err!(SubscriberEmail::try_from(email));
//     }

//     #[test]
//     fn unit_email_missing_at_symbol_is_rejected() {
//         let email = "ursuladomain.com".to_string();
//         assert_err!(SubscriberEmail::try_from(email));
//     }

//     #[test]
//     fn unit_email_missing_subject_is_rejected() {
//         let email = "@domain.com".to_string();
//         assert_err!(SubscriberEmail::try_from(email));
//     }

//     #[derive(Debug, Clone)]
//     struct ValidEmailFixture(pub String);

//     impl quickcheck::Arbitrary for ValidEmailFixture {
//         fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
//             let email = SafeEmail().fake_with_rng(g);
//             Self(email)
//         }
//     }

//     #[quickcheck_macros::quickcheck]
//     fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
//         SubscriberEmail::try_from(valid_email.0).is_ok()
//     }
// }
