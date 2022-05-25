pub mod arguments {
    #[derive(Debug)]
    pub struct Arguments {
        pub command_path: String,
        pub image_path: String,
    }
    
    impl Arguments{
        pub fn new(args: &[String]) -> Arguments {
            println!("{:?}", args.len());
            if args.len() < 2 {
                panic!("Minimum number of parameters not met");
            }
            let command_path = String::from(args[0].clone());
            let image_path = String::from(args[1].clone());
            return Arguments {
                command_path,
                image_path
            }
        }
    }

    pub fn parse_arguments(args: Vec<String>) -> Arguments {
        return Arguments::new(&args);
    }
    
    pub fn print_arguments() {
        println!("args");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_cli_arguments_into_config() {
        let args = [
            String::from("/path/of/binary"),
            String::from("/path/of/image")
        ];
        let arguments = arguments::parse_arguments(args.to_vec());
        assert_eq!(args[0], arguments.command_path);
        assert_eq!(args[1], arguments.image_path);
    }

    #[test]
    #[should_panic(expected="Minimum number of parameters not met")]
    fn panics_on_less_than_two_parameters() {
        let args = [String::from("/path/of/executable")];
        arguments::parse_arguments(args.to_vec());
    }
}
