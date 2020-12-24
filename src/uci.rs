pub struct Command {
    tokens: Vec<String>,
}

impl Command {
    pub fn from(input: &str) -> Result<Command, &str> {
        let valid_input = Self::validate_input_string(input)?;
        Ok(Command {
            tokens: valid_input
                .split_ascii_whitespace()
                .map(|x| String::from(x))
                .collect(),
        })
    }

    pub fn execute(&self) {
        print!("Command execution => ");
        for token in self.tokens.iter() {
            print!("{} + ", token);
        }
        println!("END");
    }

    fn validate_input_string(input: &str) -> Result<String, &str> {
        if input.is_empty() {
            return Err("String was empty");
        }
        Ok(String::from(input))
    }
}
