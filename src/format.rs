/// Serializes a timestamp back to a string.
/// Makes sure to add plural s at the end of the string if the nominal value is greater than 1.
pub fn format(milliseconds: i64) -> String {
    let units = [
        (86_400_000, "day"),
        (3_600_000, "hour"),
        (60_000, "minute"),
        (1000, "second"),
        (1, "ms"),
    ];

    // Loop through each of the units and find the best matching unit
    for &(unit_ms, unit_name) in units.iter() {
        if milliseconds.abs() >= unit_ms {
            // Get the converted value
            let value = milliseconds / unit_ms;

            // Add a plural s except for ms
            println!("VALUE {:?}", value.abs());
            let unit = if value.abs() > 1 && unit_name != "ms" {
                format!("{}s", unit_name)
            } else {
                unit_name.to_string()
            };

            // Format the converted value along with its new unit
            return format!("{} {}", value, unit);
        }
    }

    // Just return ms
    format!("{} ms", milliseconds)
}
