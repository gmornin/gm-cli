use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    StringErr(String),
    StrErr(&'static str),
    Unspecified,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::StringErr(e) => e,
            Self::StrErr(e) => e,
            Self::Unspecified => "unspecified error",
        })
    }
}

impl std::error::Error for Error {}
