pub fn format_uptime(uptime: u64) -> String {
    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let minutes = (uptime % 3600) / 60;
    let seconds = uptime % 60;

    format!(
        "{:02} days, {:02} hours, {:02} mins, {:02} secs",
        days, hours, minutes, seconds
    )
}
