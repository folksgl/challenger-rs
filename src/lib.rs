use std::io;
use std::sync::mpsc;

mod uci;

pub fn producer(tx: mpsc::Sender<uci::Command>) {
    loop {
        let input = get_stdin_input();

        if input == "quit" {
            // Breaking out of this loop causes the Sender end of the Channel to
            // close, which will cause the Receiver loop in `consumer` to end.
            break;
        }

        // If a valid Command can be constructed, send it to the engine
        let uci_command = match uci::Command::from(&input) {
            Ok(x) => x,
            Err(_) => continue,
        };
        tx.send(uci_command).unwrap();
    }
}

pub fn consumer(rx: mpsc::Receiver<uci::Command>) {
    for command in rx {
        command.execute();
    }
}

fn get_stdin_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Trim whitespace from the ends of the input but otherwise leave the input
    // unchanged. Invalid or malformed commands will be ignored.
    input.trim().to_string()
}
