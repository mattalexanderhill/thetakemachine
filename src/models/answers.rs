use std::fmt;
use std::iter::Iterator;

#[derive(Debug, Clone, Copy)]
pub enum Answer {
    Agree    = 1,
    Disagree = 2,
    DontKnow = 3,
    DontCare = 4
}

impl fmt::Display for Answer {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Agree    => "Agree",
            Self::Disagree => "Disagree",
            Self::DontKnow => "Don't Know",
            Self::DontCare => "Don't Care",
        })
    }
}

impl Answer {
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Agree, Self::Disagree, Self::DontKnow, Self::DontCare]
            .iter()
            .copied()
    }

    pub fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "1" => Ok(Self::Agree),
            "2" => Ok(Self::Disagree),
            "3" => Ok(Self::DontKnow),
            "4" => Ok(Self::DontCare),
            _ => Err(())
        }
    }

    pub fn from_id(id: i32) -> Self {
        match id {
            1 => Self::Agree,
            2 => Self::Disagree,
            3 => Self::DontKnow,
            4 => Self::DontCare,
            _ => Self::DontKnow
        }
    }
}
