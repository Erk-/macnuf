use core::{fmt::Display, str::FromStr};

#[derive(Copy, Clone, Debug)]
pub struct EUI48 {
    pub inner: [u8; 6],
}

impl EUI48 {
    pub fn new(inner: [u8; 6]) -> Self {
        EUI48 { inner }
    }
}

#[derive(Debug, Clone)]
pub enum EUI48ParseError {
    NotAPair,
    TooFewPairs,
    TooManyPairs,
    ParseIntError(core::num::ParseIntError),
}

impl Display for EUI48ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EUI48ParseError::NotAPair => f.write_str("Tried to parse a group that was not a pair"),
            EUI48ParseError::TooFewPairs => f.write_str("There was less than 6 groups of pairs"),
            EUI48ParseError::TooManyPairs => f.write_str("There was more than 6 groups of pairs"),
            EUI48ParseError::ParseIntError(err) => err.fmt(f),
        }
    }
}

impl core::error::Error for EUI48ParseError {}

impl FromStr for EUI48 {
    type Err = EUI48ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut eui = [0_u8; 6];

        let mut i = 0;

        for p in s.split(&[':', '-']) {
            if p.len() != 2 {
                return Err(Self::Err::NotAPair);
            }

            if i >= 6 {
                return Err(Self::Err::TooManyPairs);
            }

            let n = u8::from_str_radix(p, 16).map_err(EUI48ParseError::ParseIntError)?;

            eui[i] = n;

            i += 1;
        }

        if i < 6 {
            return Err(Self::Err::TooFewPairs);
        }

        Ok(EUI48 { inner: eui })
    }
}
