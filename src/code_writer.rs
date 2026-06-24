use std::fs::File;
use std::io::Write;

pub struct CodeWriter {
    file: File,
    filename: String,
    func_name: String,    
    label_counter: u32,
    return_counter: u32,
}

impl CodeWriter {
    pub fn new(filename: &str) -> Result<Self, std::io::Error> {
        let file = File::create(filename)?;
        let filename = filename.replace(".asm", "");
        Ok(CodeWriter {
            file,
            filename,
            func_name: String::new(),  
            label_counter: 0,
            return_counter: 0,
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
            "sub" => {
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "AM=M-1")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "A=A-1")?;
                writeln!(self.file, "M=M-D")?;
            },
            "neg" => {
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "A=M-1")?;
                writeln!(self.file, "M=-M")?;
            },
            "and" => {
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "AM=M-1")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "A=A-1")?;
                writeln!(self.file, "M=D&M")?;
            },
            "or" => {
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "AM=M-1")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "A=A-1")?;
                writeln!(self.file, "M=D|M")?;
            },
            "not" => {
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "A=M-1")?;
                writeln!(self.file, "M=!M")?;
            },
            "eq" => {
                let label_true = format!("EQ_TRUE_{}", self.label_counter);
                let label_end = format!("EQ_END_{}", self.label_counter);
                self.label_counter += 1;
                
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "AM=M-1")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "A=A-1")?;
                writeln!(self.file, "D=M-D")?;
                writeln!(self.file, "@{}", label_true)?;
                writeln!(self.file, "D;JEQ")?;
                writeln!(self.file, "D=0")?;
                writeln!(self.file, "@{}", label_end)?;
                writeln!(self.file, "0;JMP")?;
                writeln!(self.file, "({})", label_true)?;
                writeln!(self.file, "D=-1")?;
                writeln!(self.file, "({})", label_end)?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "A=M-1")?;
                writeln!(self.file, "M=D")?;
            },
            "gt" => {
                let label_true = format!("GT_TRUE_{}", self.label_counter);
                let label_end = format!("GT_END_{}", self.label_counter);
                self.label_counter += 1;
                
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "AM=M-1")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "A=A-1")?;
                writeln!(self.file, "D=M-D")?;
                writeln!(self.file, "@{}", label_true)?;
                writeln!(self.file, "D;JGT")?;
                writeln!(self.file, "D=0")?;
                writeln!(self.file, "@{}", label_end)?;
                writeln!(self.file, "0;JMP")?;
                writeln!(self.file, "({})", label_true)?;
                writeln!(self.file, "D=-1")?;
                writeln!(self.file, "({})", label_end)?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "A=M-1")?;
                writeln!(self.file, "M=D")?;
            },
            "lt" => {
                let label_true = format!("LT_TRUE_{}", self.label_counter);
                let label_end = format!("LT_END_{}", self.label_counter);
                self.label_counter += 1;
                
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "AM=M-1")?;
                writeln!(self.file, "D=M")?;
                writeln!(self.file, "A=A-1")?;
                writeln!(self.file, "D=M-D")?;
                writeln!(self.file, "@{}", label_true)?;
                writeln!(self.file, "D;JLT")?;
                writeln!(self.file, "D=0")?;
                writeln!(self.file, "@{}", label_end)?;
                writeln!(self.file, "0;JMP")?;
                writeln!(self.file, "({})", label_true)?;
                writeln!(self.file, "D=-1")?;
                writeln!(self.file, "({})", label_end)?;
                writeln!(self.file, "@SP")?;
                writeln!(self.file, "A=M-1")?;
                writeln!(self.file, "M=D")?;
            },
            _ => panic!("Operação desconhecida: {}", operation),
    }
        Ok(())
    }
    pub fn write_label(&mut self, label: &str) -> std::io::Result<()> {
        writeln!(self.file, "({}${})", self.func_name, label)?;
        Ok(())
    }
    pub fn write_goto(&mut self, label: &str) -> std::io::Result<()> {
        writeln!(self.file, "@{}${}", self.func_name, label)?;
        writeln!(self.file, "0;JMP")?;
        Ok(())
    }
    pub fn write_if(&mut self, label: &str) -> std::io::Result<()> {
        writeln!(self.file, "@SP")?;
        writeln!(self.file, "AM=M-1")?;
        writeln!(self.file, "D=M")?;
        writeln!(self.file, "@{}${}", self.func_name, label)?;
        writeln!(self.file, "D;JNE")?;
        Ok(())
    }
    pub fn write_function(&mut self, func_name: &str, n_locals: u16) -> std::io::Result<()> {
        self.func_name = func_name.to_string();

        writeln!(self.file, "({})", func_name)?;
    
        for _ in 0..n_locals {
            self.write_push("constant", 0)?;
        }
        
        Ok(())
    }
    pub fn close(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
    pub fn write_call(&mut self, func_name: &str, n_args: u16) -> std::io::Result<()> {
        let return_label = format!("{}$ret.{}", func_name, self.return_counter);
        self.return_counter += 1;

        
        writeln!(self.file, "@{}", return_label)?;
        writeln!(self.file, "D=A")?;
        writeln!(self.file, "@SP")?;
        writeln!(self.file, "A=M")?;
        writeln!(self.file, "M=D")?;
        writeln!(self.file, "@SP")?;
        writeln!(self.file, "M=M+1")?;

    
        for reg in ["LCL", "ARG", "THIS", "THAT"] {
            writeln!(self.file, "@{}", reg)?;
            writeln!(self.file, "D=M")?;
            writeln!(self.file, "@SP")?;
            writeln!(self.file, "A=M")?;
            writeln!(self.file, "M=D")?;
            writeln!(self.file, "@SP")?;
            writeln!(self.file, "M=M+1")?;
        }

   
        writeln!(self.file, "@5")?;
        writeln!(self.file, "D=A")?;
        writeln!(self.file, "@{}", n_args)?;
        writeln!(self.file, "D=D+A")?;
        writeln!(self.file, "@SP")?;
        writeln!(self.file, "D=M-D")?;
        writeln!(self.file, "@ARG")?;
        writeln!(self.file, "M=D")?;


        writeln!(self.file, "@SP")?;
        writeln!(self.file, "D=M")?;
        writeln!(self.file, "@LCL")?;
        writeln!(self.file, "M=D")?;

  
        writeln!(self.file, "@{}", func_name)?;
        writeln!(self.file, "0;JMP")?;

        
        writeln!(self.file, "({})", return_label)?;

        Ok(())
    }
    pub fn write_return(&mut self) -> std::io::Result<()> {
        
        writeln!(self.file, "@LCL")?;
        writeln!(self.file, "D=M")?;
        writeln!(self.file, "@R13")?;
        writeln!(self.file, "M=D")?;

       
        writeln!(self.file, "@5")?;
        writeln!(self.file, "A=D-A")?;
        writeln!(self.file, "D=M")?;
        writeln!(self.file, "@R14")?;
        writeln!(self.file, "M=D")?;

      
        writeln!(self.file, "@SP")?;
        writeln!(self.file, "AM=M-1")?;
        writeln!(self.file, "D=M")?;
        writeln!(self.file, "@ARG")?;
        writeln!(self.file, "A=M")?;
        writeln!(self.file, "M=D")?;

      
        writeln!(self.file, "@ARG")?;
        writeln!(self.file, "D=M")?;
        writeln!(self.file, "@SP")?;
        writeln!(self.file, "M=D+1")?;

       
        writeln!(self.file, "@R13")?;
        writeln!(self.file, "AM=M-1")?;
        writeln!(self.file, "D=M")?;
        writeln!(self.file, "@THAT")?;
        writeln!(self.file, "M=D")?;

    
        writeln!(self.file, "@R13")?;
        writeln!(self.file, "AM=M-1")?;
        writeln!(self.file, "D=M")?;
        writeln!(self.file, "@THIS")?;
        writeln!(self.file, "M=D")?;

   
        writeln!(self.file, "@R13")?;
        writeln!(self.file, "AM=M-1")?;
        writeln!(self.file, "D=M")?;
        writeln!(self.file, "@ARG")?;
        writeln!(self.file, "M=D")?;


        writeln!(self.file, "@R13")?;
        writeln!(self.file, "AM=M-1")?;
        writeln!(self.file, "D=M")?;
        writeln!(self.file, "@LCL")?;
        writeln!(self.file, "M=D")?;

   
        writeln!(self.file, "@R14")?;
        writeln!(self.file, "A=M")?;
        writeln!(self.file, "0;JMP")?;

        Ok(())
    }
}