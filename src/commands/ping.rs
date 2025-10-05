#[allow(dead_code)]

pub fn execute(args: &[&str]) -> String {
    if args.is_empty() {
        "+PONG\r\n".to_string()
    } else {
        format!("+{}\r\n", args[0])
    }
}
