pub mod parse;
pub mod format;

pub use parse::parse;
pub use format::format;

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
}

