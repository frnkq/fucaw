pub struct Arguments {
    pub command_path: String,
    pub image_path: String,
}

impl Arguments {
    pub fn new(args: &[String]) -> Arguments {
        let command_path = String::from(args[0].clone());
        let image_path = String::from(args[1].clone());
        return Arguments {
            command_path,
            image_path,
        };
    }
}

pub fn parse_arguments(args: Vec<String>) -> Option<Arguments> {
    if args.len() < 2 {
        return None;
    }
    let arguments = Arguments::new(&args);
    return Some(arguments);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_cli_arguments_into_config() {
        let args = [
            String::from("/path/of/binary"),
            String::from("/path/of/image"),
        ];
        let arguments = parse_arguments(args.to_vec()).unwrap();
        assert_eq!(args[0], arguments.command_path);
        assert_eq!(args[1], arguments.image_path);
    }

    #[test]
    fn should_return_none_if_less_than_two_arguments() {
        let args = [String::from("/path/of/executable")];
        let arguments = parse_arguments(args.to_vec());
        assert_eq!(arguments.is_some(), false);
    }
}
