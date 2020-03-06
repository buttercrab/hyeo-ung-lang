use std::{collections::HashMap, error, fmt, num, result};

pub enum Error {
    ParseError,
    NoVersion,
    BadRequest,
}

impl error::Error for Error {}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Error::ParseError => "Parse Error",
            Error::NoVersion => "Version Not Found",
            Error::BadRequest => "Bad Internet Connection",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Error::ParseError => "Parse Error",
            Error::NoVersion => "Version Not Found",
            Error::BadRequest => "Bad Internet Connection",
        };
        write!(f, "{}", s)
    }
}

pub type Version = (u16, u16, u16);
pub type Result = result::Result<Version, Error>;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn parse_version(version: &str) -> Result {
    let f = || {
        let v = version.split(".").collect::<Vec<&str>>();
        result::Result::Ok((
            v[0].parse::<u16>()?,
            v[1].parse::<u16>()?,
            v[2].parse::<u16>()?,
        ))
    };
    f().map_err(|_: num::ParseIntError| Error::ParseError)
}

pub fn get_current_version() -> Version {
    parse_version(VERSION).unwrap()
}

pub async fn get_latest_version() -> Result {
    let res: HashMap<String, String> = async {
        result::Result::<HashMap<String, String>, reqwest::Error>::Ok(
            reqwest::get("https://api.github.com/repos/buttercrab/hyeo-ung-lang/releases/latest")
                .await?
                .json::<HashMap<String, String>>()
                .await?,
        )
    }
    .await
    .map_err(|_| Error::BadRequest)?;

    match res.get(&*String::from("tag_name")) {
        Some(ref version) => parse_version(&version.as_str()[1..]),
        None => Result::Err(Error::NoVersion),
    }
}

pub async fn get_update_version(version: &str) -> Result {
    if version == "latest" {
        get_latest_version().await
    } else {
        parse_version(version)
    }
}
