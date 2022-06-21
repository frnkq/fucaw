pub struct Arguments {
    pub source_image_path: String,
}

impl Arguments {
    pub fn new(args: &[String]) -> Arguments {
        let source_image_path = String::from(args[1].clone());
        return Arguments { source_image_path };
    }
}

pub fn parse_arguments(args: Vec<String>) -> Arguments {
    let arguments = Arguments::new(&args);
    return arguments;
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
        let arguments = parse_arguments(args.to_vec());
        assert_eq!(args[1], arguments.source_image_path);
    }
}
