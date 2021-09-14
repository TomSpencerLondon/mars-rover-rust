pub struct Rover {
}

impl Rover {
    pub fn new() -> Rover {
        Rover { }
    }

    pub fn execute(&mut self, input: &str) -> String {
        String::from("0:0:N")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_command() {
        let mut rover: Rover = Rover::new();
        assert_eq!(rover.execute(""), String::from("0:0:N"));
    }
}
