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
        let input = input.trim();

        // Match the input against the UCI
        let uci_regex_set =
            RegexSet::new(&[
                r"^(uci|isready|ucinewgame|stop|ponderhit|quit)$",
                r"^debug (on|off)$",
                r"^position (startpos|([rnbqkp12345678RNBQKP]{1,8}/){7}[rnbqkp12345678RNBQKP]{1,8} (w|b) (-|[KQkq]{1,4}) (-|[a-h][1-8]) (\\d)+ (\\d)+)( moves( [a-h][1-8][a-h][1-8][rnbqRNBQ]?)+)?$",
                r"^go( ponder| infinite| (wtime|btime|winc|binc|movestogo|depth|nodes|mate|movetime) [\\d]+| searchmoves( [a-h][1-8][a-h][1-8][rnbqRNBQ]?)+)*$",
                r"^setoption [[:word:]]+( value [[:word:]]+)?$"
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

    // Valid uci
    test_valid_command!(valid_uci_1, "uci", vec!["uci"]);
    test_valid_command!(valid_uci_2, "\nuci", vec!["uci"]);
    test_valid_command!(valid_uci_3, "\tuci", vec!["uci"]);
    test_valid_command!(valid_uci_4, "\n\t   uci\n\n\t\t\n ", vec!["uci"]);

    // Invalid uci
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

    // Valid debug
    test_valid_command!(valid_debug_1, "debug on", vec!["debug", "on"]);
    test_valid_command!(valid_debug_2, "debug off", vec!["debug", "off"]);

    // Invalid debug
    test_invalid_command!(invalid_debug_1, "ddebug on");
    test_invalid_command!(invalid_debug_2, "debug o");
    test_invalid_command!(invalid_debug_3, "debug onn");
    test_invalid_command!(invalid_debug_4, "ebug on");
    test_invalid_command!(invalid_debug_5, "debug");
    test_invalid_command!(invalid_debug_6, "debug on off");
    test_invalid_command!(invalid_debug_7, "debug off on");
    test_invalid_command!(invalid_debug_8, "debug onoff");
    test_invalid_command!(invalid_debug_9, "asdf");
    test_invalid_command!(invalid_debug_10, "asdf debug on");
    test_invalid_command!(invalid_debug_11, "debug on asdf");
    test_invalid_command!(invalid_debug_12, "d\nebug on");
    test_invalid_command!(invalid_debug_13, "^debug on");
    test_invalid_command!(invalid_debug_14, "debug off$");
    test_invalid_command!(invalid_debug_15, "debug\noff");

    // Valid isready
    test_valid_command!(valid_isready_1, "isready", vec!["isready"]);

    // Invalid isready
    test_invalid_command!(invalid_isready_1, "iisready");
    test_invalid_command!(invalid_isready_2, "isreadyy");
    test_invalid_command!(invalid_isready_3, "is ready");
    test_invalid_command!(invalid_isready_4, "a isready");
    test_invalid_command!(invalid_isready_5, "isready a");
    test_invalid_command!(invalid_isready_6, "asdfisready");
    test_invalid_command!(invalid_isready_7, "isreadyasdf");
    test_invalid_command!(invalid_isready_8, "sready");
    test_invalid_command!(invalid_isready_9, "i\nsready");
    test_invalid_command!(invalid_isready_10, "i\tsready");
    test_invalid_command!(invalid_isready_11, "isready isready");
    test_invalid_command!(invalid_isready_12, "isready$");
    test_invalid_command!(invalid_isready_13, "^isready");
    test_invalid_command!(invalid_isready_14, "isready\nisready");

    // Valid setoption
    test_valid_command!(
        valid_setoption_1,
        "setoption name value x",
        vec!["setoption", "name", "value", "x"]
    );
    test_valid_command!(
        valid_setoption_2,
        "setoption name value 1",
        vec!["setoption", "name", "value", "1"]
    );
    test_valid_command!(
        valid_setoption_3,
        "setoption asdf_1234",
        vec!["setoption", "asdf_1234"]
    );
    test_valid_command!(
        valid_setoption_4,
        "setoption asdf_1234 value asdf_1234",
        vec!["setoption", "asdf_1234", "value", "asdf_1234"]
    );

    // Invalid setoption
    test_invalid_command!(invalid_setoption_1, "isetoption");
    test_invalid_command!(invalid_setoption_2, "setoptiony");
    test_invalid_command!(invalid_setoption_3, "set option");
    test_invalid_command!(invalid_setoption_4, "setoptionn name value x");
    test_invalid_command!(invalid_setoption_5, "ssetoption name value x");
    test_invalid_command!(invalid_setoption_6, "etoption asdf");
    test_invalid_command!(invalid_setoption_7, "setoption value 42");
    test_invalid_command!(invalid_setoption_8, "setoption 42 24");
    test_invalid_command!(invalid_setoption_9, "setoption\n name value x");

    // Valid ucinewgame
    test_valid_command!(valid_ucinewgame_1, "ucinewgame", vec!["ucinewgame"]);

    // Invalid ucinewgame
    test_invalid_command!(invalid_ucinewgame_1, "uucinewgame");
    test_invalid_command!(invalid_ucinewgame_2, "ucinewgamee");
    test_invalid_command!(invalid_ucinewgame_3, "uci newgame");
    test_invalid_command!(invalid_ucinewgame_4, "asdf");
    test_invalid_command!(invalid_ucinewgame_5, "cinewgame");
    test_invalid_command!(invalid_ucinewgame_6, "ucinewgam");
    test_invalid_command!(invalid_ucinewgame_7, "ucinew\ngame");
    test_invalid_command!(invalid_ucinewgame_8, "ucinewgameucinewgame");
    test_invalid_command!(invalid_ucinewgame_9, "ucinewgame ucinewgame");
    test_invalid_command!(invalid_ucinewgame_10, "^ucinewgame");
    test_invalid_command!(invalid_ucinewgame_11, "ucinewgame$");

    // Valid position
    // Invalid position

    // Valid go
    //test_valid_command!(valid__1, "", vec![""]);
    // Invalid go

    // Valid stop
    //test_valid_command!(valid__1, "", vec![""]);
    // Invalid stop

    // Valid ponderhit
    //test_valid_command!(valid__1, "", vec![""]);
    // Invalid ponderhit
}
