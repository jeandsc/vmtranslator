use std::fs::File;
use std::io::Write;

pub struct CodeWriter {
    file: File,
    filename: String,
}

impl CodeWriter {
    pub fn new(filename: &str) -> Result<Self, std::io::Error> {
        let file = File::create(filename)?;
        let filename = filename.replace(".asm", "");
        Ok(CodeWriter {
            file,
            filename,
        })
    }
    
    pub fn write_push(&mut self, segment: &str, index: u16) -> std::io::Result<()> {
        match segment {
            "constant" => {
                writeln!(self.file, "@{}", index)?;
                writeln!(self.file, "D=A")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "A=M")?;
                writeln!(self.file, "M=D")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "M=M+1")?;
            },
            "local" => {
                writeln!(self.file, "@{}", index)?;
                writeln!(self.file, "D=A")?;
                writeln!(self.file, "@LCL")?;
                writeln!(self.file, "A=M+D")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "A=M")?;
                writeln!(self.file, "M=D")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "M=M+1")?;
            },
            "argument" => {
                writeln!(self.file, "@{}", index)?;
                writeln!(self.file, "D=A")?;
                writeln!(self.file, "@ARG")?;
                writeln!(self.file, "A=M+D")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "A=M")?;
                writeln!(self.file, "M=D")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "M=M+1")?;
            },
            "this" => {
                writeln!(self.file, "@{}", index)?;
                writeln!(self.file, "D=A")?;
                writeln!(self.file, "@THIS")?;
                writeln!(self.file, "A=M+D")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "A=M")?;
                writeln!(self.file, "M=D")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "M=M+1")?;
            },
            "that" => {
                writeln!(self.file, "@{}", index)?;
                writeln!(self.file, "D=A")?;
                writeln!(self.file, "@THAT")?;
                writeln!(self.file, "A=M+D")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "A=M")?;
                writeln!(self.file, "M=D")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "M=M+1")?;
            },
            "temp" => {
                writeln!(self.file, "@{}", 5 + index)?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "A=M")?;
                writeln!(self.file, "M=D")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "M=M+1")?;
            },
            "pointer" => {
                writeln!(self.file, "@{}", 3 + index)?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "A=M")?;
                writeln!(self.file, "M=D")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "M=M+1")?;
            },
            "static" => {
                writeln!(self.file, "@{}.{}", self.filename, index)?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "A=M")?;
                writeln!(self.file, "M=D")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "M=M+1")?;
            },
            _ => panic!("Segmento desconhecido: {}", segment),
        }
        Ok(())
    }
    
    pub fn write_pop(&mut self, segment: &str, index: u16) -> std::io::Result<()> {
        match segment {
            "constant" => {
                panic!("Não pode pop constant");
            },
            "local" => {
                writeln!(self.file, "@{}", index)?;
                writeln!(self.file, "D=A")?;
                writeln!(self.file, "@LCL")?;
                writeln!(self.file, "D=M+D")?;
                writeln!(self.file, "@R13")?;
                writeln!(self.file, "M=D")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "AM=M-1")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "@R13")?;
                writeln!(self.file, "A=M")?;
                writeln!(self.file, "M=D")?;
            },
            "argument" => {
                writeln!(self.file, "@{}", index)?;
                writeln!(self.file, "D=A")?;
                writeln!(self.file, "@ARG")?;
                writeln!(self.file, "D=M+D")?;
                writeln!(self.file, "@R13")?;
                writeln!(self.file, "M=D")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "AM=M-1")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "@R13")?;
                writeln!(self.file, "A=M")?;
                writeln!(self.file, "M=D")?;
            },
            "this" => {
                writeln!(self.file, "@{}", index)?;
                writeln!(self.file, "D=A")?;
                writeln!(self.file, "@THIS")?;
                writeln!(self.file, "D=M+D")?;
                writeln!(self.file, "@R13")?;
                writeln!(self.file, "M=D")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "AM=M-1")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "@R13")?;
                writeln!(self.file, "A=M")?;
                writeln!(self.file, "M=D")?;
            },
            "that" => {
                writeln!(self.file, "@{}", index)?;
                writeln!(self.file, "D=A")?;
                writeln!(self.file, "@THAT")?;
                writeln!(self.file, "D=M+D")?;
                writeln!(self.file, "@R13")?;
                writeln!(self.file, "M=D")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "AM=M-1")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "@R13")?;
                writeln!(self.file, "A=M")?;
                writeln!(self.file, "M=D")?;
            },
            "temp" => {
                writeln!(self.file, "@{}", 5 + index)?;
                writeln!(self.file, "D=A")?;
                writeln!(self.file, "@R13")?;
                writeln!(self.file, "M=D")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "AM=M-1")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "@R13")?;
                writeln!(self.file, "A=M")?;
                writeln!(self.file, "M=D")?;
            },
            "pointer" => {
                writeln!(self.file, "@{}", 3 + index)?;
                writeln!(self.file, "D=A")?;
                writeln!(self.file, "@R13")?;
                writeln!(self.file, "M=D")?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "AM=M-1")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "@R13")?;
                writeln!(self.file, "A=M")?;
                writeln!(self.file, "M=D")?;
            },
            "static" => {
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "AM=M-1")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "@{}.{}", self.filename, index)?;
                writeln!(self.file, "M=D")?;
            },
            _ => panic!("Segmento desconhecido: {}", segment),
        }
        Ok(())
    }
    
    pub fn write_arithmetic(&mut self, operation: &str) -> std::io::Result<()> {
        match operation {
            "add" => {
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "AM=M-1")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "A=A-1")?;
                writeln!(self.file, "M=D+M")?;
            },
            _ => panic!("Operação desconhecida: {}", operation),
    }
        Ok(())
    }
    
    pub fn close(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}