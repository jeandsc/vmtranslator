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
        Ok(())
    }
    
    pub fn write_arithmetic(&mut self, operation: &str) -> std::io::Result<()> {
        Ok(())
    }
    
    pub fn close(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}