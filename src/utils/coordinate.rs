use std::str::FromStr;

#[derive(Debug)]
pub enum PointParseError {
    WrongSeparator,
    TokenError,
}

#[derive(Clone, Copy)]
pub struct Point2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2d<T>
where
    T: FromStr,
{
    pub fn parse_csv(s: &str) -> Result<Self, PointParseError> {
        let tokens = s.split_once(',');
        if let Some((x, y)) = tokens {
            let parse = |inp: &str| {
                inp.trim()
                    .parse::<T>()
                    .map_err(|_| PointParseError::TokenError)
            };
            let x = parse(x)?;
            let y = parse(y)?;
            Ok(Self { x, y })
        } else {
            Err(PointParseError::WrongSeparator)
        }
    }
}
