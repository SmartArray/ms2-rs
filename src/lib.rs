pub mod parse;
pub mod format;

pub use parse::parse;
pub use format::format;

pub fn ms<T: Into<MsInput>>(input: T) -> Result<MsOutput, &'static str> {
    match input.into() {
        MsInput::Str(s) => parse(&s).map(MsOutput::Milliseconds),
        MsInput::Int(i) => Ok(MsOutput::Str(format(i))),
    }
}

pub enum MsInput {
    Str(String),
    Int(u64),
}

impl From<&str> for MsInput {
    fn from(s: &str) -> Self {
        MsInput::Str(s.to_string())
    }
}

impl From<String> for MsInput {
    fn from(s: String) -> Self {
        MsInput::Str(s)
    }
}

impl From<u64> for MsInput {
    fn from(i: u64) -> Self {
        MsInput::Int(i)
    }
}

pub enum MsOutput {
    Str(String),
    Milliseconds(u64),
}

// Some basic tests for this library.
// ToDo: Add more tests here. I needed to craft those quickly.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("2 days").unwrap(), 172_800_000);
        assert_eq!(parse("1 minute").unwrap(), 60_000);
        assert!(parse("unknown").is_err());
    }

    #[test]
    fn test_format() {
        assert_eq!(format(172_800_000), "2 days");
        assert_eq!(format(60_000), "1 minute");
        assert_eq!(format(999), "999 ms");
    }

    #[test]
    fn test_ms_wrapper() {
        match ms("2 days").unwrap() {
            MsOutput::Milliseconds(ms) => assert_eq!(ms, 172_800_000),
            _ => panic!("Expected milliseconds"),
        }
        match ms(60_000).unwrap() {
            MsOutput::Str(s) => assert_eq!(s, "1 minute"),
            _ => panic!("Expected string"),
        }
        assert!(ms("unknown").is_err());
    }    
}
