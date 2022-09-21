use thiserror::Error;

#[derive(Error, Debug)]
pub enum DepartmentError {
    #[error("Unknown department")]
    Unkown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Department {
    Som,
    Jsys,
    Honki,
    Kosen,
    Stage,
    Sok,
    Dan,
    Ss,
}

impl TryFrom<&str> for Department {
    type Error = DepartmentError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let lower = s.to_lowercase();

        match lower.as_str() {
            "som" => Ok(Department::Som),
            "jsys" => Ok(Department::Jsys),
            "honki" => Ok(Department::Honki),
            "kosen" => Ok(Department::Kosen),
            "stage" => Ok(Department::Stage),
            "sok" => Ok(Department::Sok),
            "dan" => Ok(Department::Dan),
            "ss" => Ok(Department::Ss),
            _ => Err(DepartmentError::Unkown),
        }
    }
}

impl std::fmt::Display for Department {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Department::Som => write!(f, "som"),
            Department::Jsys => write!(f, "jsys"),
            Department::Honki => write!(f, "honki"),
            Department::Kosen => write!(f, "kosen"),
            Department::Stage => write!(f, "stage"),
            Department::Sok => write!(f, "sok"),
            Department::Dan => write!(f, "dan"),
            Department::Ss => write!(f, "ss"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        assert!(Department::try_from("som").is_ok());
        assert!(Department::try_from("jsys").is_ok());
        assert!(Department::try_from("honki").is_ok());
        assert!(Department::try_from("kosen").is_ok());
        assert!(Department::try_from("stage").is_ok());
        assert!(Department::try_from("sok").is_ok());
        assert!(Department::try_from("dan").is_ok());
        assert!(Department::try_from("ss").is_ok());
    }

    #[test]
    fn invalid() {
        assert!(Department::try_from("invalidDepartment").is_err());
    }
}
