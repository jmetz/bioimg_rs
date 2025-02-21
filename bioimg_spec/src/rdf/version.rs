use std::{fmt::Display, num::ParseIntError};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum VersionParsingError {
    #[error("Expected 3 fields, found {found}")]
    WrongNumberOfComponents { found: usize },
    #[error("Could not parse version field: {0}")]
    ParseIntError(ParseIntError),
    #[error("Expected version '{expected}', found '{found}'")]
    UnexpectedVersion { expected: Version, found: Version },
}
impl From<ParseIntError> for VersionParsingError {
    fn from(value: ParseIntError) -> Self {
        return Self::ParseIntError(value);
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct Version {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
}
impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_string: String = self.clone().into();
        write!(f, "{self_string}",)
    }
}
impl TryFrom<&str> for Version {
    type Error = VersionParsingError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value
            .split(".")
            .map(|comp| comp.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;
        if parts.len() != 3 {
            return Err(VersionParsingError::WrongNumberOfComponents { found: parts.len() });
        }
        return Ok(Version {
            major: parts[0],
            minor: parts[1],
            patch: parts[2],
        });
    }
}
impl TryFrom<String> for Version {
    type Error = VersionParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        return <Self as TryFrom<&str>>::try_from(&value);
    }
}

impl Into<String> for Version {
    fn into(self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[test]
fn test_version_parsing() {
    use serde_json::Value as JsonValue;

    let raw_version = JsonValue::String("1.2.3".into());

    assert_eq!(
        serde_json::from_value::<Version>(raw_version).unwrap(),
        Version {
            major: 1,
            minor: 2,
            patch: 3
        }
    );
    assert_eq!(
        Version::try_from("1.2"),
        Err(VersionParsingError::WrongNumberOfComponents { found: 2 })
    );
    assert_eq!(
        Version::try_from("1.2.bla"),
        Err(VersionParsingError::ParseIntError(
            "bla".parse::<u32>().expect_err("should fail parsing")
        ))
    );
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(try_from = "Version")]
#[serde(into = "Version")]
pub struct LiteralVersion<const MAJOR: usize, const MINOR: usize, const PATCH: usize>;

impl<const MAJOR: usize, const MINOR: usize, const PATCH: usize> Into<Version> for LiteralVersion<MAJOR, MINOR, PATCH> {
    fn into(self) -> Version {
        return Version {
            major: MAJOR,
            minor: MINOR,
            patch: PATCH,
        };
    }
}

impl<const MAJOR: usize, const MINOR: usize, const PATCH: usize> TryFrom<Version> for LiteralVersion<MAJOR, MINOR, PATCH> {
    type Error = VersionParsingError;

    fn try_from(value: Version) -> Result<Self, Self::Error> {
        if value.major == MAJOR && value.minor == MINOR && value.patch == PATCH {
            Ok(Self)
        } else {
            Err(VersionParsingError::UnexpectedVersion {
                expected: Self.into(),
                found: value,
            })
        }
    }
}
