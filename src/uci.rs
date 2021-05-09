use regex::RegexSet;
use std::io;
use std::sync::mpsc;
use std::thread;

// Begin accepting UCI commands from stdin. This is the entry point for running
// Challenger. All game actions and modifications begin from stdin.
pub fn start_uci_engine() {
    let (sender, receiver) = mpsc::channel();

    let producer_handle = thread::spawn(move || producer(sender));
    let consumer_handle = thread::spawn(move || consumer(receiver));

    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();
}

// Commands represent valid UCI commands entered by a user. Only valid commands
// should ever be sent to the Challenger engine to execute, so user input MUST
// be validated before the '.execute()' method is called by the engine.
struct Command {
    input_string: String,
}

impl Command {
    pub fn from(input: &str) -> Result<Command, &str> {
        let valid_input = validate_input_string(input)?;
        Ok(Command {
            input_string: valid_input,
        })
    }

    pub fn execute(&self) {
        match self.tokens()[0] {
            "uci" => println!("id name Challenger\nid author folksgl\nuciok"),
            _ => println!("something else"),
        }
    }

    pub fn tokens(&self) -> Vec<&str> {
        return self.input_string.split_whitespace().collect();
    }
}

// Validate that the input is a well-formed UCI command string. Return the
// command tokens in a vector, or Err() if invalid.
fn validate_input_string(input: &str) -> Result<String, &str> {
    let input = input.trim();

    // Match the input against known Universal Chess Interface (UCI) commands
    let uci_regex_set =
            RegexSet::new(&[
                r"^(?:uci|isready|ucinewgame|stop|ponderhit)$",
                r"^debug (?:on|off)$",
                r"^position (?:startpos|(?:[rnbqkp12345678RNBQKP]{1,8}/){7}[rnbqkp12345678RNBQKP]{1,8} (w|b) (?:-|[KQkq]{1,4}) (?:-|[a-h][1-8]) (?:\d)+ (?:\d)+)(?: moves(?: [a-h][1-8][a-h][1-8][rnbqRNBQ]?)+)?$",
                r"^go(?: ponder| infinite| (?:wtime|btime|winc|binc|movestogo|depth|nodes|mate|movetime) [\d]+| searchmoves(?: [a-h][1-8][a-h][1-8][rnbqRNBQ]?)+)*$",
                r"^setoption [[:word:]]+(?: value [[:word:]]+)?$"
            ]).unwrap();

    if uci_regex_set.is_match(&input) {
        Ok(String::from(input))
    } else {
        Err("Command failed UCI regex validation")
    }
}

// "Produces" Commands by parsing stdin input and sending the resulting
// Command struct to the consuming mpsc::Receiver
fn producer(tx: mpsc::Sender<Command>) {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let input = buffer.trim();

        if input == "quit" {
            // Breaking out of this loop causes the Sender end of the Channel to
            // close, which will cause the Receiver loop in `consumer` to end.
            break;
        }

        // If a valid Command can be constructed, send it to the engine
        let uci_command = match Command::from(&input) {
            Ok(x) => x,
            Err(_) => continue,
        };
        tx.send(uci_command).unwrap();
    }
}

// "Consumes" Commands by reading from the mpsc::Receiver and executing
// the received Command.
fn consumer(rx: mpsc::Receiver<Command>) {
    for command in rx {
        command.execute();
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // Macro for defining tests that validate good input strings against a known
    // set of tokens that should be returned by that input.
    macro_rules! test_valid_command {
        ($test_name:ident, $input_str:literal) => {
            #[test]
            fn $test_name() {
                assert!(validate_input_string($input_str).is_ok());
            }
        };
    }

    // Macro for defining tests that should NOT create a command, and should
    // instead panic on receiving an Err(str) from validate_string(str)
    macro_rules! test_invalid_command {
        ($test_name:ident, $input_str:literal) => {
            #[test]
            fn $test_name() {
                assert!(validate_input_string($input_str).is_err());
            }
        };
    }

    // Valid uci
    test_valid_command!(valid_uci_1, "uci");
    test_valid_command!(valid_uci_2, "\nuci");
    test_valid_command!(valid_uci_3, "\tuci");
    test_valid_command!(valid_uci_4, "\n\t   uci\n\n\t\t\n ");

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
    test_valid_command!(valid_debug_1, "debug on");
    test_valid_command!(valid_debug_2, "debug off");

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
    test_valid_command!(valid_isready_1, "isready");

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
    test_valid_command!(valid_setoption_1, "setoption name value x");
    test_valid_command!(valid_setoption_2, "setoption name value 1");
    test_valid_command!(valid_setoption_3, "setoption asdf_1234");
    test_valid_command!(valid_setoption_4, "setoption asdf_1234 value asdf_1234");

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
    test_valid_command!(valid_ucinewgame_1, "ucinewgame");

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
    test_valid_command!(valid_position_1, "position startpos");
    test_valid_command!(valid_position_2, "position 8/8/8/8/8/8/8/8 w KQkq - 0 1");
    test_valid_command!(
        valid_position_3,
        "position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    );
    test_valid_command!(
        valid_position_4,
        "position rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
    );
    test_valid_command!(
        valid_position_5,
        "position rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2"
    );
    test_valid_command!(
        valid_position_6,
        "position rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2"
    );
    test_valid_command!(
        valid_position_7,
        "position rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq a1 1 2"
    );
    test_valid_command!(
        valid_position_8,
        "position rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b Qkq a1 1 2"
    );
    test_valid_command!(
        valid_position_9,
        "position rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b kq a1 1 2"
    );
    test_valid_command!(
        valid_position_10,
        "position rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b q a1 1 2"
    );
    test_valid_command!(
        valid_position_11,
        "position rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b - a1 1 2"
    );
    test_valid_command!(valid_position_12, "position rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1234567890987654321 2234567890987654321");
    test_valid_command!(valid_position_13, "position startpos moves a1a2");
    test_valid_command!(valid_position_14, "position startpos moves a1a2 b2b2");
    test_valid_command!(valid_position_15, "position startpos moves a1a2 b2b2 c3c3");
    test_valid_command!(
        valid_position_16,
        "position startpos moves a1a2 b2b2 c3c3 d4d4"
    );
    test_valid_command!(
        valid_position_17,
        "position startpos moves a1a2 b2b2 c3c3 d4d4q"
    );
    test_valid_command!(
        valid_position_18,
        "position startpos moves a1a2 b2b2 c3c3 d4d4Q"
    );
    test_valid_command!(
        valid_position_19,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 1 moves e1e2"
    );
    test_valid_command!(
        valid_position_20,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 1 moves f1e8 d5f8"
    );
    test_valid_command!(
        valid_position_21,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 1 moves g1e6 d5f8"
    );
    test_valid_command!(
        valid_position_22,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 1 moves h1e8 d5f8q"
    );
    test_valid_command!(
        valid_position_23,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 1 moves e1e5 d5f8Q"
    );
    test_valid_command!(
        valid_position_24,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 1 moves e1e7 d5f8n"
    );
    test_valid_command!(
        valid_position_25,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 1 moves e1e7 d5f8N"
    );
    test_valid_command!(
        valid_position_26,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 1 moves e1e7 d5f8r"
    );
    test_valid_command!(
        valid_position_27,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 1 moves e1e7 d5f8R"
    );
    test_valid_command!(
        valid_position_28,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 1 moves e1e7 d5f8b"
    );
    test_valid_command!(
        valid_position_29,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 1 moves e1e7 d5f8B"
    );
    test_valid_command!(
        valid_position_30,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 1 moves d5f8B"
    );
    // Invalid position
    test_invalid_command!(invalid_position_1, "uposition");
    test_invalid_command!(invalid_position_2, "positione");
    test_invalid_command!(invalid_position_3, "posit on");
    test_invalid_command!(invalid_position_4, "asdf");
    test_invalid_command!(invalid_position_5, "\n\n");
    test_invalid_command!(invalid_position_6, "osition");
    test_invalid_command!(invalid_position_7, "startpos");
    test_invalid_command!(invalid_position_8, "position 8/8/8/8/8/8/8/8 w KQkq - 0");
    test_invalid_command!(invalid_position_9, "position 8/8/8/8/8/8/8/8 w KQkq - 0 -");
    test_invalid_command!(
        invalid_position_10,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 w)"
    );
    test_invalid_command!(invalid_position_11, "position 8/8/8/8/8/8/8/8 - KQkq - 0 0");
    test_invalid_command!(invalid_position_12, "position 8/8/8/8/8/8/8/u w KQkq - 0 0");
    test_invalid_command!(invalid_position_13, "position 8/8/8/8/8/8/8/8 w tQkq - 0 0");
    test_invalid_command!(invalid_position_14, "position 8/8/8/8/8/8/8/8 w Ktkq - 0 0");
    test_invalid_command!(invalid_position_15, "position 8/8/8/8/8/8/8/8 w KQtq - 0 0");
    test_invalid_command!(invalid_position_16, "position 8/8/8/8/8/8/8/8 w KQkt - 0 0");
    test_invalid_command!(invalid_position_17, "position 8/8/8/8/8/8/8/8 w KQkq a 0 0");
    test_invalid_command!(invalid_position_18, "position 8/8/8/8/8/8/8/8 w KQkq 1 0 0");
    test_invalid_command!(invalid_position_19, "position 8/8/8/8/8//8/8 w KQkq - 0 0");
    test_invalid_command!(invalid_position_20, "position 8/8/8/8/8/8/8/8 w - 0 0");
    test_invalid_command!(invalid_position_21, "position startpos move a1a2");
    test_invalid_command!(invalid_position_22, "position startpos moves i1a2");
    test_invalid_command!(invalid_position_23, "position startpos moves a1j2");
    test_invalid_command!(invalid_position_24, "position startpos moves a0a2");
    test_invalid_command!(invalid_position_25, "position startpos moves a9a2");
    test_invalid_command!(invalid_position_26, "position startpos moves a1a0");
    test_invalid_command!(invalid_position_27, "position startpos moves a1a9");
    test_invalid_command!(invalid_position_28, "position startpos moves a1a1qq");
    test_invalid_command!(invalid_position_29, "position startpos moves a1a1W");
    test_invalid_command!(invalid_position_30, "position startpos moves a1a1w");
    test_invalid_command!(
        invalid_position_31,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 move a1a2"
    );
    test_invalid_command!(
        invalid_position_32,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 0 moves i1a2"
    );
    test_invalid_command!(
        invalid_position_33,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 0 moves a1j2"
    );
    test_invalid_command!(
        invalid_position_34,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 0 moves a0a2"
    );
    test_invalid_command!(
        invalid_position_35,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 0 moves a9a2"
    );
    test_invalid_command!(
        invalid_position_36,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 0 moves a1a0"
    );
    test_invalid_command!(
        invalid_position_37,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 0 moves a1a9"
    );
    test_invalid_command!(
        invalid_position_38,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 0 moves a1a1qq"
    );
    test_invalid_command!(
        invalid_position_39,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 0 moves a1a1W"
    );
    test_invalid_command!(
        invalid_position_40,
        "position 8/8/8/8/8/8/8/8 w KQkq - 0 0 moves a1a1w"
    );

    // Valid go
    test_valid_command!(valid_go_1, "go");
    test_valid_command!(valid_go_2, "go depth 1");
    test_valid_command!(valid_go_3, "go depth 1234567890");
    test_valid_command!(valid_go_4, "go depth 1 ponder");
    test_valid_command!(valid_go_5, "go depth 3 wtime 4");
    test_valid_command!(valid_go_6, "go nodes 7");
    test_valid_command!(valid_go_7, "go mate 09");
    test_valid_command!(valid_go_8, "go infinite searchmoves a1a2 a2a4q");
    // Invalid go
    test_invalid_command!(invalid_go_1, "ugo");
    test_invalid_command!(invalid_go_2, "gon");
    test_invalid_command!(invalid_go_3, "g\no");
    test_invalid_command!(invalid_go_4, "g o");
    test_invalid_command!(invalid_go_5, "\n\n");
    test_invalid_command!(invalid_go_6, "o");
    test_invalid_command!(invalid_go_7, "g");
    test_invalid_command!(invalid_go_8, "go depth");
    test_invalid_command!(invalid_go_9, "go depth infinite");
    test_invalid_command!(invalid_go_10, "go depth a");
    test_invalid_command!(invalid_go_11, "go winc");
    test_invalid_command!(invalid_go_12, "go winc p");
    test_invalid_command!(invalid_go_13, "go movestogo");
    test_invalid_command!(invalid_go_14, "go winc binc 4");
    test_invalid_command!(invalid_go_15, "go inc 4");

    // Valid stop
    test_valid_command!(valid_stop_1, "stop");
    // Invalid stop
    test_invalid_command!(invalid_stop_1, "sstop");
    test_invalid_command!(invalid_stop_2, "stopp");
    test_invalid_command!(invalid_stop_3, "st\nop");
    test_invalid_command!(invalid_stop_4, "st\top");
    test_invalid_command!(invalid_stop_5, "sto");
    test_invalid_command!(invalid_stop_6, "top");
    test_invalid_command!(invalid_stop_7, "st op");
    test_invalid_command!(invalid_stop_8, "stop stop");
    test_invalid_command!(invalid_stop_9, "stopstop");
    test_invalid_command!(invalid_stop_10, "1stop");
    test_invalid_command!(invalid_stop_11, "astop");
    test_invalid_command!(invalid_stop_12, "stop1");
    test_invalid_command!(invalid_stop_13, "stop 1");
    test_invalid_command!(invalid_stop_14, "st p");
    test_invalid_command!(invalid_stop_15, "s op");
    test_invalid_command!(invalid_stop_16, "sto p");
    test_invalid_command!(invalid_stop_17, "^stop");
    test_invalid_command!(invalid_stop_18, "stop$");

    // Valid ponderhit
    test_valid_command!(valid_ponderhit_1, "ponderhit");
    // Invalid ponderhit
    test_invalid_command!(invalid_ponderhit_1, "pponderhit");
    test_invalid_command!(invalid_ponderhit_2, "ponderhitt");
    test_invalid_command!(invalid_ponderhit_3, "ponder\nhit");
    test_invalid_command!(invalid_ponderhit_4, "ponder\thit");
    test_invalid_command!(invalid_ponderhit_5, "ponderhi");
    test_invalid_command!(invalid_ponderhit_6, "onderhit");
    test_invalid_command!(invalid_ponderhit_7, "ponder hit");
    test_invalid_command!(invalid_ponderhit_8, "ponderhitponderhit");
    test_invalid_command!(invalid_ponderhit_9, "ponderhit ponderhit");
    test_invalid_command!(invalid_ponderhit_10, "p onderhit");
    test_invalid_command!(invalid_ponderhit_11, "go ponderhit");
    test_invalid_command!(invalid_ponderhit_12, "ponderhit isready");
    test_invalid_command!(invalid_ponderhit_13, "^ponderhit");
    test_invalid_command!(invalid_ponderhit_14, "ponderhit$");
    test_invalid_command!(invalid_ponderhit_15, "ponderhit\nisready");

    // Test command creation (does Command::tokens get properly populated)
    macro_rules! test_command_tokens {
        ($test_name:ident, $input_str:literal, $expected:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!(Command::from($input_str).unwrap().tokens(), $expected)
            }
        };
    }

    test_command_tokens!(uci_tokens, "uci", vec!["uci"]);
    test_command_tokens!(isready_tokens, "isready", vec!["isready"]);
    test_command_tokens!(ucinewgame_tokens, "ucinewgame", vec!["ucinewgame"]);
    test_command_tokens!(stop_tokens, "stop", vec!["stop"]);
    test_command_tokens!(ponderhit_tokens, "ponderhit", vec!["ponderhit"]);
    test_command_tokens!(
        position_tokens_1,
        "position startpos",
        vec!["position", "startpos"]
    );
    test_command_tokens!(
        position_tokens_2,
        "position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        vec![
            "position",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
            "w",
            "KQkq",
            "-",
            "0",
            "1"
        ]
    );
    test_command_tokens!(
        position_tokens_3,
        "position 8/8/8/8/8/8/8/8 b - - 0 0",
        vec!["position", "8/8/8/8/8/8/8/8", "b", "-", "-", "0", "0"]
    );
    test_command_tokens!(
        position_tokens_4,
        "position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves a1a2 b4b8R",
        vec![
            "position",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
            "w",
            "KQkq",
            "-",
            "0",
            "1",
            "moves",
            "a1a2",
            "b4b8R"
        ]
    );
    test_command_tokens!(
        position_tokens_5,
        "position startpos moves a2a4 h7h5 f2f8Q",
        vec!["position", "startpos", "moves", "a2a4", "h7h5", "f2f8Q"]
    );
    test_command_tokens!(go_tokens, "go depth 2", vec!["go", "depth", "2"]);
    test_command_tokens!(
        go_tokens_2,
        "go depth 2 wtime 123 btime 321",
        vec!["go", "depth", "2", "wtime", "123", "btime", "321"]
    );
    test_command_tokens!(
        go_tokens_3,
        "go depth 2 infinite ponder",
        vec!["go", "depth", "2", "infinite", "ponder"]
    );
    test_command_tokens!(debug_tokens, "debug on", vec!["debug", "on"]);
    test_command_tokens!(
        setoption_tokens,
        "setoption myoption value 4",
        vec!["setoption", "myoption", "value", "4"]
    );
}
