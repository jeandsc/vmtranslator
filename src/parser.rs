//usando enum e struct para os comandos 
enum CommandType {
    CPush,
    CPop,
    CArithmetic,
}
struct ParsedCommand {
    cmd_type: CommandType,
    arg1: String,
    arg2: Option<u16>,
}


pub struct Parser {
    commands: Vec<ParsedCommand>,  
    index: usize,
    current: Option<ParsedCommand>, 
}

