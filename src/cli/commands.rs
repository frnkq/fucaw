use std::io::Result;
use std::process::Command;
use std::process::Output;

pub fn read_history_file(file_path: &str) -> Result<Output> {
    let bash_history = Command::new("cat").arg(file_path).output();
    return bash_history;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_history_file() {
        let status = read_history_file("~/.bash_history");
        assert_eq!(true, status.is_ok());
    }
}
