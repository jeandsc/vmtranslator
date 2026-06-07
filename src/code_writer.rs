use std::fs::File;
use std::io::Write;

pub struct CodeWriter {
    file: File,
}

impl AsmWriter {
    pub fn new(filename: &str) -> Result<Self, std::io::Error> {
        let file = File::create(filename)?;
        Ok(AsmWriter {
            file,
        })
    }
    
    pub fn write_push(&mut self, segment: &str, index: u16) -> std::io::Result<()> {
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