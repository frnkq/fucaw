use std::process::Command;

const HISTORY_FILE_PATH: &str = "/home/frnkq/.bash_history";
const MAX_NUMBER_OF_COMMANDS: usize = 25;

#[derive(Debug, Clone)]
pub struct Cmd {
    command: String,
    occurrence: i32,
}

impl Default for Cmd {
   fn default()-> Self {
    Self { command: Default::default(), occurrence: Default::default()}
   }
}

pub fn read_terminal_history() -> Option<String> {
    let history = Command::new("cat").arg(HISTORY_FILE_PATH).output();
    match history.is_ok() {
        true => Some(String::from_utf8(history.unwrap().stdout).unwrap()),
        false => None,
    }
}

pub fn filter_commands(history: Vec<String>) -> Vec<Cmd> {
    let mut freq: Vec<Cmd> = vec![];

    for command in history {
        let cmd = Cmd { 
            command: command,
            occurrence: 0
        };

        match get_index_of(&cmd, &freq) {
            Some(index) =>  freq[index].occurrence += 1,
            None => freq.push(cmd)
        }
    }

    freq.sort_by(|a,b| b.occurrence.cmp(&a.occurrence));

    if freq.len() > MAX_NUMBER_OF_COMMANDS {
        freq.resize_with(MAX_NUMBER_OF_COMMANDS, Default::default)
    }

    return freq;
}

pub fn frequently_used() -> Option<Vec<Cmd>> {
    match read_terminal_history() {
        Some(history) => {
            let commands: Vec<String> = history
                .split("\n")
                .map(|cmd| cmd.split(" ").next().unwrap().to_string())
                .collect();
            Some(filter_commands(commands))
        },
        None => None
    }
}

fn get_index_of(command: &Cmd, in_vector: &Vec<Cmd>) -> Option<usize> {
    let mut index: usize = 0;
    for el in in_vector.iter() {
        if el.command == command.command {
            return Some(index);
        }
        index +=1;
    }
    return None;
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
        let commands = String::from("commandA\ncommandB\ncommandC\ncommandB\ncommandC\ncommandB")
            .split("\n")
            .map(|cmd| cmd.split(" ").next().unwrap().to_string())
            .collect();
        let command_lines: String = filter_commands(commands).into_iter().map(|cmd| cmd.command.to_string()).collect();
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
        assert_eq!(2, get_index_of(&command, &vec).unwrap());
    }
}
