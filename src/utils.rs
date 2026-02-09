pub fn format_duration(diff: chrono::Duration) -> String {
    let secs = diff.num_seconds();
    if secs < 60 {
        format!("({}s ago)", secs)
    } else if secs < 3600 {
        format!("({}m ago)", secs / 60)
    } else if secs < 3600 * 24 {
        format!("({}h ago)", secs / 3600)
    } else {
        format!("({}d ago)", secs / (3600 * 24))
    }
}
