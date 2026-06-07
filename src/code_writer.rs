use std::fs::File;
use std::io::Write;

pub struct CodeWriter {
    file: File,
}

impl CodeWriter {
    pub fn new(filename: &str) -> Result<Self, std::io::Error> {
        let file = File::create(filename)?;
        Ok(CodeWriter {
            file,
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