//usando enum e struct para os comandos 
#[derive(Debug, PartialEq, Clone)] 
pub enum CommandType {
    CPush,
    CPop,
    CArithmetic,
}
#[derive(Debug, PartialEq, Clone)] 
pub struct ParsedCommand {
    pub cmd_type: CommandType,
    pub arg1: String,
    pub arg2: Option<u16>,
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
                "push" => {
                    let segment = tokens[1];
                    if !Self::validate_segment(segment) {
                        panic!("Segmento inválido: {}", segment);
                    }
                    ParsedCommand {

                    cmd_type: CommandType::CPush,
                    arg1: tokens[1].to_string(),
                    arg2: Some(tokens[2].parse().unwrap()),
                    }

                },
                "pop" => {
                    let segment = tokens[1];
                    if !Self::validate_segment(segment) {
                        panic!("Segmento inválido: {}", segment);
                    }
                    if segment == "constant" {
                        panic!("Segmento inválido: {}", segment);
                    }
                    ParsedCommand {

                    cmd_type: CommandType::CPop,
                    arg1: tokens[1].to_string(),
                    arg2: Some(tokens[2].parse().unwrap()),
                    }
                },
                "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => {
                    ParsedCommand {
                        cmd_type: CommandType::CArithmetic,
                        arg1: tokens[0].to_string(),
                        arg2: None,
                    }
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
    pub fn has_more_commands(&self) -> bool {
        self.index < self.commands.len()
    }
    
    pub fn advance(&mut self) {
        self.current = Some(self.commands[self.index].clone());
        self.index += 1;
    }
    pub fn current(&self) -> &ParsedCommand {
        self.current.as_ref().unwrap()
    }
    fn validate_segment(segment: &str) -> bool {
        match segment {
            "constant" | "local" | "argument" | "this" | "that" | "temp" | "pointer" | "static" => true,
            _ => false,
        }
    }
}
