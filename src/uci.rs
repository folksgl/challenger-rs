use regex::RegexSet;

pub struct Command {
    tokens: Vec<String>,
}

impl Command {
    pub fn from(input: &str) -> Result<Command, &str> {
        let valid_input = Self::validate_input_string(input)?;
        Ok(Command {
            tokens: valid_input,
        })
    }

    pub fn execute(&self) {
        match self.tokens[0].as_str() {
            "uci" => println!("id name Challenger\nid author folksgl\nuciok"),
            _ => println!("something else"),
        }
        print!("Command execution => ");
        for token in self.tokens.iter() {
            print!("{} + ", token);
        }
        println!("END");
    }

    fn validate_input_string(input: &str) -> Result<Vec<String>, &str> {
        // Turn the input into an str of space-separated words
        let input = input.split_whitespace().collect::<Vec<&str>>().join(" ");

        // Match the input against the UCI
        let uci_regex_set =
            RegexSet::new(&[
                r"^(uci|isready|ucinewgame|stop|ponderhit|quit)$",
                r"^position (startpos|([rnbqkp12345678RNBQKP]{1,8}/){7}[rnbqkp12345678RNBQKP]{1,8} (w|b) (-|[KQkq]{1,4}) (-|[a-h][1-8]) (\\d)+ (\\d)+)( moves( [a-h][1-8][a-h][1-8][rnbqRNBQ]?)+)?$",
                r"^go( ponder| infinite| (wtime|btime|winc|binc|movestogo|depth|nodes|mate|movetime) [\\d]+| searchmoves( [a-h][1-8][a-h][1-8][rnbqRNBQ]?)+)*$",
                r"^setoption [\\w]+( value [\\w]+)?$"
            ]).unwrap();

        if uci_regex_set.is_match(&input) {
            let valid = input.split_whitespace().map(|x| String::from(x)).collect();
            Ok(valid)
        } else {
            Err("Command failed UCI regex validation")
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn validate_string(input: &str) -> Vec<String> {
        return Command::validate_input_string(input).unwrap();
    }

    // Macro for defining tests that validate good input strings against a known
    // set of tokens that should be returned by that input.
    macro_rules! test_valid_command {
        ($test_name:ident, $input_str:literal, $expected:expr) => {
            #[test]
            fn $test_name() {
                let result = validate_string($input_str);
                assert_eq!(result, $expected);
            }
        };
    }

    // Macro for defining tests that should NOT create a command, and should
    // instead panic on receiving an Err(str) from validate_string(str)
    macro_rules! test_invalid_command {
        ($test_name:ident, $input_str:literal) => {
            #[test]
            #[should_panic]
            fn $test_name() {
                let _result = validate_string($input_str);
            }
        };
    }

    // Valid UCI commands
    test_valid_command!(valid_uci_1, "uci", vec!["uci"]);
    test_valid_command!(valid_uci_2, "\nuci", vec!["uci"]);
    test_valid_command!(valid_uci_3, "\tuci", vec!["uci"]);
    test_valid_command!(valid_uci_4, "\n\t   uci\n\n\t\t\n ", vec!["uci"]);

    // Invalid UCI commands
    test_invalid_command!(invalid_uci_1, "ci");
    test_invalid_command!(invalid_uci_2, "uuci");
    test_invalid_command!(invalid_uci_3, "ucii");
    test_invalid_command!(invalid_uci_4, "uci asdf");
    test_invalid_command!(invalid_uci_5, "uciasdf");
    test_invalid_command!(invalid_uci_6, "asdfuci");
    test_invalid_command!(invalid_uci_7, "asdf uci");
    test_invalid_command!(invalid_uci_8, "1uci");
    test_invalid_command!(invalid_uci_9, "1 uci");
    test_invalid_command!(invalid_uci_10, "u ci");
    test_invalid_command!(invalid_uci_11, "$uci");
    test_invalid_command!(invalid_uci_12, "^uci");
    test_invalid_command!(invalid_uci_13, "^uci");
}
