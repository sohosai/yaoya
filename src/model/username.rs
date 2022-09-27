use super::department::{Department, DepartmentError};
use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum UsernameError {
    #[error("Username doesn't start with the year digits")]
    MissingYear,
    #[error("Username implies unknown department")]
    UnknownDepartment(String),
    #[error(
        "Username is malformed. Year: {0}, Department: {1}, Family name: {2}, Given name: {3}"
    )]
    Malformed(u8, Department, String, String),
    #[error("Username contains invalid characters")]
    InvalidCharacters,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Username {
    pub year: u8,
    pub department: Department,
    pub family_name: String,
    pub given_name: String,
}

const UPPER_LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

impl TryFrom<&str> for Username {
    type Error = UsernameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(UsernameError::InvalidCharacters);
        }

        let year = &value[0..2];
        if !year.chars().all(|c| c.is_ascii_digit()) {
            return Err(UsernameError::MissingYear);
        }

        let year = year.parse::<u8>().unwrap();
        let department = value[2..]
            .chars()
            .take_while(|c| !UPPER_LETTERS.contains(*c))
            .collect::<String>();

        let department = match Department::try_from(department.as_str()) {
            Ok(d) => d,
            Err(DepartmentError::Unkown) => {
                return Err(UsernameError::UnknownDepartment(department))
            }
        };

        let family_name = value[2 + department.to_string().len()..]
            .chars()
            .enumerate()
            .take_while(|(i, c)| *i == 0 || !UPPER_LETTERS.contains(*c))
            .map(|(_, c)| c)
            .collect::<String>();

        let given_name = value[2 + department.to_string().len() + family_name.len()..].to_string();

        if family_name.is_empty() || given_name.is_empty() {
            return Err(UsernameError::Malformed(
                year,
                department,
                family_name,
                given_name,
            ));
        }

        Ok(Username {
            year,
            department,
            family_name,
            given_name,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_valid_usernames() {
        // 4 letters department
        assert_eq!(
            Username::try_from("21jsysItoYusei"),
            Ok(Username {
                year: 21,
                department: Department::Jsys,
                family_name: "Ito".to_string(),
                given_name: "Yusei".to_string(),
            })
        );

        // 3 letters department
        assert_eq!(
            Username::try_from("21danAokiSouta"),
            Ok(Username {
                year: 21,
                department: Department::Dan,
                family_name: "Aoki".to_string(),
                given_name: "Souta".to_string()
            })
        );
    }

    #[test]
    fn parse_invalid_usernames() {
        assert_eq!(
            Username::try_from("21jsysItoYusei!"),
            Err(UsernameError::InvalidCharacters)
        );
        assert_eq!(
            Username::try_from("jsysItoYusei21"),
            Err(UsernameError::MissingYear)
        );

        assert_eq!(
            Username::try_from("21fooItoYusei"),
            Err(UsernameError::UnknownDepartment("foo".to_string()))
        );

        assert_eq!(
            Username::try_from("21jsysItoyusei"),
            Err(UsernameError::Malformed(
                21,
                Department::Jsys,
                "Itoyusei".to_string(),
                String::new()
            ))
        );
    }
}
