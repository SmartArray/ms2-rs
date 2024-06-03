use std::collections::HashMap;

/// This function parses an existing input (such as "5 minutes") and returns
/// the value in miliseconds.
pub fn parse(input: &str) -> Result<u64, &'static str> {
    let mut units: HashMap<&str, u64> = HashMap::new();
    units.insert("ms", 1);
    units.insert("second", 1000);
    units.insert("seconds", 1000);
    units.insert("minute", 60_000);
    units.insert("minutes", 60_000);
    units.insert("hour", 3_600_000);
    units.insert("hours", 3_600_000);
    units.insert("day", 86_400_000);
    units.insert("days", 86_400_000);

    // Calling trim on the input
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() != 2 {
        return Err("Invalid input format. Use format like '2 days'.");
    }

    // Try to parse number
    let value: u64 = tokens[0].parse().map_err(|_| "Invalid number")?;
    let unit = tokens[1];

    match units.get(unit) {
        Some(&multiplier) => Ok(value * multiplier),
        None => Err("Unknown time unit"),
    }
}
