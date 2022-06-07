use std::io::Result;
use std::process::Command;
use std::process::Output;

const HISTORY_FILE_PATH: &str = "/home/frnkq/.bash_history";
const MAX_NUMBER_OF_COMMANDS: usize = 10;

pub fn read_terminal_history() -> Result<Output> {
    let bash_history = Command::new("cat").arg(HISTORY_FILE_PATH).output();
    return bash_history;
}

pub fn filter_commands(command_history: Vec<String>) -> Vec<String> {
    let mut commands: Vec<String> = command_history.clone();
    if commands.len() > MAX_NUMBER_OF_COMMANDS {
        commands.resize_with(MAX_NUMBER_OF_COMMANDS, Default::default);
    }
    return commands;
}

pub fn frequently_used() -> Option<Vec<String>> {
    let history = String::from_utf8(read_terminal_history().unwrap().stdout).unwrap();

    if history.is_empty() {
        None
    } else {
        let mut commands: Vec<String> = history.split("\n").map(|s| s.to_string()).collect();
        Some(filter_commands(commands))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_history_file() {
        let status = read_terminal_history();
        assert_eq!(true, status.is_ok());
    }

    #[test]
    fn gets_MAX_NUMBER_OF_COMMANDS_or_less() {
        let command_lines = frequently_used().unwrap();
        assert_eq!(true, command_lines.len() <= MAX_NUMBER_OF_COMMANDS);
    }
}
