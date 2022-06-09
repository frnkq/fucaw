use std::io::Result;
use std::process::Command;

const HISTORY_FILE_PATH: &str = "/home/frnkq/.bash_history";
const MAX_NUMBER_OF_COMMANDS: usize = 10;

pub fn read_terminal_history() -> Result<String> {
    let history = Command::new("cat").arg(HISTORY_FILE_PATH).output();
    match history.is_ok() {
        true => Ok(String::from_utf8(history.unwrap().stdout).unwrap()),
        false => Ok(String::from("")),
    }
}

pub fn filter_commands(history: String) -> Vec<String> {
    let mut commands: Vec<String> = history
        .split("\n")
        .map(|s| s.to_string())
        .map(|s| s.split(" ").next().unwrap().to_string())
        .collect();

    commands.sort();
    commands.dedup();

    if commands.len() > MAX_NUMBER_OF_COMMANDS {
        commands.resize_with(MAX_NUMBER_OF_COMMANDS, Default::default)
    }

    print!("{:?}", commands);
    return commands;
}

pub fn frequently_used() -> Option<Vec<String>> {
    let history = read_terminal_history().unwrap();

    if history.is_empty() {
        None
    } else {
        Some(filter_commands(history))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_history_file() {
        let status = read_terminal_history().unwrap();
        assert_eq!(false, status.is_empty());
    }

    #[test]
    fn gets_max_number_of_commands_or_less() {
        let command_lines = frequently_used().unwrap();
        assert_eq!(true, command_lines.len() <= MAX_NUMBER_OF_COMMANDS);
    }

    #[test]
    fn filters_duplicate_commands() {
        let commands = String::from("commandA\ncommandB\ncommandC\ncommandB\ncommandC\ncommandB");
        let command_lines: String = filter_commands(commands).into_iter().collect();
        assert_eq!(1, command_lines.matches("commandB").count());
        assert_eq!(1, command_lines.matches("commandC").count());
    }

    #[test]
    fn sorts_by_occurrences() {
        let commands = String::from("commandA\ncommandB\ncommandC\ncommandA\ncommandA\ncommandC");
        let expected: Vec<String> = String::from("commandA\ncommandC\ncommandB")
            .split("\n")
            .map(|s| s.to_string())
            .collect();

        assert_eq!(expected, filter_commands(commands));
    }
}
