// the result of parsing one line of input

pub struct Command<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str>,
}

// parses a trimmed, non - empty input string into a command
// returns none if input is empty
pub fn parse(input: &str) -> Option<Command<'_>> {
    let mut parts = input.split_whitespace();
    let name = parts.next()?;
    let args = parts.collect();
    Some(Command { name, args })
}



