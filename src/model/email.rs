use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UniveristyEmailAddressError {
    #[error("Malformed Email Address. Reason: {0}")]
    MalformedEMailAdderess(email_address::Error),
    #[error("Is not university email address (so-called S-address)")]
    IsNotUniversityEmail,
}

#[derive(Debug)]
pub struct UniversityEmailAddress {
    email: email_address::EmailAddress,
}

impl TryFrom<&str> for UniversityEmailAddress {
    type Error = UniveristyEmailAddressError;
    fn try_from(email: &str) -> Result<Self, Self::Error> {
        let normalized_address = email.to_lowercase();
        let validated_address = match email_address::EmailAddress::from_str(&normalized_address) {
            Ok(email) => email,
            Err(e) => return Err(UniveristyEmailAddressError::MalformedEMailAdderess(e)),
        };

        if validated_address.domain().eq("s.tsukuba.ac.jp") {
            Ok(UniversityEmailAddress {
                email: validated_address,
            })
        } else {
            Err(UniveristyEmailAddressError::IsNotUniversityEmail)
        }
    }
}
