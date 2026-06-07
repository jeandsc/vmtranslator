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

impl Parser {
    pub fn new(filename: &str) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(filename)?;
        let mut commands = Vec::new();
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            
            let line = line.split("//").next().unwrap().trim();
            let tokens: Vec<&str> = line.split_whitespace().collect();
            
            
            let cmd = match tokens[0] {
                "push" => ParsedCommand {
                    cmd_type: CommandType::CPush,
                    arg1: tokens[1].to_string(),
                    arg2: Some(tokens[2].parse().unwrap()),
                },
                _ => panic!("Comando desconhecido: {}", tokens[0]),
            };
            
            commands.push(cmd);
        }
        
        Ok(Parser {
            commands,
            index: 0,
            current: None,
        })
    }
}
