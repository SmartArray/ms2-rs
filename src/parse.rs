use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Define the units statically
static UNITS: Lazy<HashMap<&str, u64>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("ms", 1);
    m.insert("second", 1000);
    m.insert("seconds", 1000);
    m.insert("minute", 60_000);
    m.insert("minutes", 60_000);
    m.insert("hour", 3_600_000);
    m.insert("hours", 3_600_000);
    m.insert("day", 86_400_000);
    m.insert("days", 86_400_000);
    m
});

/// This function parses an existing input (such as "5 minutes") and returns
/// the value in miliseconds.
pub fn parse(input: &str) -> Result<u64, &'static str> {
    // Calling trim on the input
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() != 2 {
        return Err("Invalid input format. Use format like '2 days'.");
    }

    // Try to parse number
    let value: u64 = tokens[0].parse().map_err(|_| "Invalid number")?;
    let unit = tokens[1];

    match UNITS.get(unit) {
        Some(&multiplier) => Ok(value * multiplier),
        None => Err("Unknown time unit"),
    }
}
