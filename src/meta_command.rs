use std::process::exit;

pub enum MetaResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

pub fn deal_with_meta_command(command: &str) -> Option<MetaResult> {
    match command {
        ".exit" => exit(0),
        _ => Some(MetaResult::MetaCommandUnrecognizedCommand),
    }
}
