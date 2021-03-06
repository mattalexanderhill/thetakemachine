use std::io;
use std::fmt;

use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::insert_into;

use diesel::backend::Backend;
use diesel::pg::PgConnection;

use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, ToSql, Output};

use crate::db::schema::demographics;


#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[sql_type="Integer"]
pub enum Gender {
    Woman = 1,
    Man   = 2,
    Other = 3,
}

impl Gender {
    pub fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "1" => Ok(Self::Woman),
            "2" => Ok(Self::Man),
            "3" => Ok(Self::Other),
            _ => Err(()),
        }
    }
}

pub const GENDER_OPTIONS: [Gender; 3] = [
    Gender::Woman,
    Gender::Man,
    Gender::Other,
];

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Woman => "Woman",
            Self::Man   => "Man",
            Self::Other => "I identify another way",
        })
    }
}

impl<DB> ToSql<Integer, DB> for Gender
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: io::Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        let v = match *self {
            Self::Woman => 1,
            Self::Man   => 2,
            Self::Other => 3,
        };
        v.to_sql(out)
    }
}

impl<DB> FromSql<Integer, DB> for Gender
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            1 => Ok(Self::Woman),
            2 => Ok(Self::Man),
            3 => Ok(Self::Other),
            x => Err(format!("Unrecognized gender_id {}", x).into()),
        }
    }
}

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[sql_type="Integer"]
pub enum AgeGroup {
    LessThan18 = 1,
    From18to25 = 2,
    From25to34 = 3,
    From35to44 = 5,
    From45to54 = 6,
    From55to64 = 7,
    MoreThan65 = 8,
}

pub const AGE_GROUP_OPTIONS: [AgeGroup; 7] = [
        AgeGroup::LessThan18,
        AgeGroup::From18to25,
        AgeGroup::From25to34,
        AgeGroup::From35to44,
        AgeGroup::From45to54,
        AgeGroup::From55to64,
        AgeGroup::MoreThan65,
];

impl AgeGroup {
    pub fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "1" => Ok(Self::LessThan18),
            "2" => Ok(Self::From18to25),
            "3" => Ok(Self::From25to34),
            "5" => Ok(Self::From35to44),
            "6" => Ok(Self::From45to54),
            "7" => Ok(Self::From55to64),
            "8" => Ok(Self::MoreThan65),
            _ => Err(()),
        }
    }
}

impl fmt::Display for AgeGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::LessThan18 => "<18",
            Self::From18to25 => "18-24",
            Self::From25to34 => "25-34",
            Self::From35to44 => "35-44",
            Self::From45to54 => "45-54",
            Self::From55to64 => "55-64",
            Self::MoreThan65 => "65+",
        })
    }
}

impl<DB> ToSql<Integer, DB> for AgeGroup
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: io::Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        let v = match *self {
            Self::LessThan18 => 1,
            Self::From18to25 => 2,
            Self::From25to34 => 3,
            Self::From35to44 => 5,
            Self::From45to54 => 6,
            Self::From55to64 => 7,
            Self::MoreThan65 => 8,
        };
        v.to_sql(out)
    }
}

impl<DB> FromSql<Integer, DB> for AgeGroup
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            1 => Ok(Self::LessThan18),
            2 => Ok(Self::From18to25),
            3 => Ok(Self::From25to34),
            5 => Ok(Self::From35to44),
            6 => Ok(Self::From45to54),
            7 => Ok(Self::From55to64),
            8 => Ok(Self::MoreThan65),
            x => Err(format!("Unrecognized age_group_id {}", x).into()),
        }
    }
}

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[sql_type="Integer"]
pub enum Politics {
    Ineligible   = 1,
    Labour       = 2,
    Conservative = 3,
    LibDem       = 4,
    Green        = 5,
    Brexit       = 6,
    Snp          = 7,
    Other        = 8,
    DidNotVote   = 9,
}

pub const POLITICS_OPTIONS: [Politics; 9] = [
    Politics::Ineligible,
    Politics::Labour,
    Politics::Conservative,
    Politics::LibDem,
    Politics::Green,
    Politics::Brexit,
    Politics::Snp,
    Politics::Other,
    Politics::DidNotVote,
];

impl Politics {
    pub fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "1" => Ok(Self::Ineligible),
            "2" => Ok(Self::Labour),
            "3" => Ok(Self::Conservative),
            "4" => Ok(Self::LibDem),
            "5" => Ok(Self::Green),
            "6" => Ok(Self::Brexit),
            "7" => Ok(Self::Snp),
            "8" => Ok(Self::Other),
            "9" => Ok(Self::DidNotVote),
            _ => Err(())
        }
    }
}


impl fmt::Display for Politics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Ineligible   => "I was not eligible to vote",
            Self::Labour       => "Labour",
            Self::Conservative => "Conservative",
            Self::LibDem       => "Liberal Democrats",
            Self::Green        => "Green",
            Self::Brexit       => "The Brexit Party",
            Self::Snp          => "Scottish National Party",
            Self::Other        => "Other",
            Self::DidNotVote   => "I did not vote",
        })
    }
}

impl<DB> ToSql<Integer, DB> for Politics
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: io::Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        let v = match *self {
            Self::Ineligible   => 1,
            Self::Labour       => 2,
            Self::Conservative => 3,
            Self::LibDem       => 4,
            Self::Green        => 5,
            Self::Brexit       => 6,
            Self::Snp          => 7,
            Self::Other        => 8,
            Self::DidNotVote   => 9,
        };
        v.to_sql(out)
    }
}

impl<DB> FromSql<Integer, DB> for Politics
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            1 => Ok(Self::Ineligible),
            2 => Ok(Self::Labour),
            3 => Ok(Self::Conservative),
            4 => Ok(Self::LibDem),
            5 => Ok(Self::Green),
            6 => Ok(Self::Brexit),
            7 => Ok(Self::Snp),
            8 => Ok(Self::Other),
            9 => Ok(Self::DidNotVote),
            x => Err(format!("Unrecognized politics_id {}", x).into()),
        }
    }
}

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[sql_type="Integer"]
pub enum Ethics {
    Cooperate = 1,
    Defect    = 2,
    DontKnow  = 3,
}

pub const ETHICS_OPTIONS: [Ethics; 3] = [
    Ethics::Cooperate,
    Ethics::Defect,
    Ethics::DontKnow,
];

impl Ethics {
    pub fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "1" => Ok(Self::Cooperate),
            "2" => Ok(Self::Defect),
            "3" => Ok(Self::DontKnow),
            _ => Err(()),
        }
    }
}


impl fmt::Display for Ethics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Cooperate => "Cooperate",
            Self::Defect    => "Defect",
            Self::DontKnow  => "I don't know",
        })
    }
}

impl<DB> ToSql<Integer, DB> for Ethics
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<W: io::Write>(&self, out: &mut Output<W, DB>) -> serialize::Result {
        let v = match *self {
            Self::Cooperate => 1,
            Self::Defect    => 2,
            Self::DontKnow  => 3,
        };
        v.to_sql(out)
    }
}

impl<DB> FromSql<Integer, DB> for Ethics
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            1 => Ok(Self::Cooperate),
            2 => Ok(Self::Defect),
            3 => Ok(Self::DontKnow),
            x => Err(format!("Unrecognized ethics_id {}", x).into()),
        }
    }
}

#[derive(Debug, Queryable, Insertable)]
pub struct Demographic {
    pub session_id: String,
    pub gender: Option<Gender>,
    pub age_group: Option<AgeGroup>,
    pub politics: Option<Politics>,
    pub ethics: Option<Ethics>,
}

pub fn store_demographic(conn: &PgConnection, dem: &Demographic) -> QueryResult<usize> {
    insert_into(demographics::table).values(dem).execute(conn)
}
