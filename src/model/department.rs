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
    Sg,
    Zai,
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
            "sg" => Ok(Department::Sg),
            "zai" => Ok(Department::Zai),
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
            Department::Sg => write!(f, "sg"),
            Department::Zai => write!(f, "zai"),
        }
    }
}

impl Department {
    pub fn to_japanese(self) -> &'static str {
        match self {
            Department::Som => "総務局",
            Department::Jsys => "情報メディアシステム局",
            Department::Honki => "本部企画局",
            Department::Kosen => "広報宣伝局",
            Department::Stage => "ステージ管理局",
            Department::Sok => "総合計画局",
            Department::Dan => "委員長団",
            Department::Ss => "推進局",
            Department::Sg => "渉外局",
            Department::Zai => "財務局",
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
        assert!(Department::try_from("sg").is_ok());
        assert!(Department::try_from("zai").is_ok());
    }

    #[test]
    fn invalid() {
        assert!(Department::try_from("invalidDepartment").is_err());
    }
}
