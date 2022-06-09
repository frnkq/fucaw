use std::io::Result;
use std::process::Command;

const HISTORY_FILE_PATH: &str = "/home/frnkq/.bash_history";
const MAX_NUMBER_OF_COMMANDS: usize = 10;

#[derive(Debug, Clone)]
struct Cmd {
    command: String,
    occurrence: i32,
}

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
        .map(|cmd| cmd.split(" ").next().unwrap().to_string())
        .collect();

    commands.sort();
    commands.dedup();

    // let mut freq: Vec<Cmd> = vec![];
    // for command in commands {
    //     if get_index_of(&command, &freq) != -1 {
    //         freq.push(command);
    //     }
    // }

    if commands.len() > MAX_NUMBER_OF_COMMANDS {
        commands.resize_with(MAX_NUMBER_OF_COMMANDS, Default::default)
    }

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

fn get_index_of(command: &Cmd, in_vector: &Vec<Cmd>) -> i32 {
    let mut i = 0;
    let mut index: i32 = -1;
    for el in in_vector.iter() {
        if el.command == command.command {
            index = i;
            break;
        }
        i += 1;
    }
    return index;
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
    fn gets_index_of_cmd_in_vec() {
        let commands_str: String = String::from("commandA\ncommandB\ncommandC\ncommandB\n");
        let command = Cmd {
            command: String::from("commandC"),
            occurrence: 0,
        };

        let vec: Vec<Cmd> = commands_str
            .split("\n")
            .map(|c| {
                let cmd = Cmd {
                    command: c.to_string(),
                    occurrence: 1,
                };
                return cmd;
            })
            .collect();
        assert_eq!(2, get_index_of(&command, &vec));
    }
}
