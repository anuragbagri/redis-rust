pub mod ping;
pub mod pong;

pub fn handle_command(input: &str) -> String {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return "-ERR empty command\r\n".to_string();
    }

    match parts[0].to_uppercase().as_str() {
        "PING" => ping::execute(&parts[1..]),
        _ => "-Err unknown command\r\n".to_string(),
    }
}
