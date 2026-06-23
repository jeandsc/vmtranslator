mod parser;
mod code_writer;

use parser::Parser;
use code_writer::CodeWriter;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Uso: vmtranslator arquivo.vm");
        process::exit(1);
    }
    
    let input = &args[1];
    let output = input.replace(".vm", ".asm");
    
    println!("Traduzindo: {} -> {}", input, output);
    
    let mut parser = match Parser::new(input) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Erro ao ler arquivo {}: {}", input, e);
            process::exit(1);
        }
    };
    
    let mut writer = match CodeWriter::new(&output) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Erro ao criar arquivo {}: {}", output, e);
            process::exit(1);
        }
    };
    
    while parser.has_more_commands() {
        parser.advance();
        let cmd = parser.current();
        
        match cmd.cmd_type {
            parser::CommandType::CPush => {
                if let Err(e) = writer.write_push(&cmd.arg1, cmd.arg2.unwrap()) {
                    eprintln!("Erro ao escrever push: {}", e);
                    process::exit(1);
                }
            },
            parser::CommandType::CPop => {
                if let Err(e) = writer.write_pop(&cmd.arg1, cmd.arg2.unwrap()) {
                    eprintln!("Erro ao escrever pop: {}", e);
                    process::exit(1);
                }
            },
            parser::CommandType::CArithmetic => {
                if let Err(e) = writer.write_arithmetic(&cmd.arg1) {
                    eprintln!("Erro ao escrever aritmético: {}", e);
                    process::exit(1);
                }
            },
            parser::CommandType::CLabel => {
                println!("label {}", cmd.arg1);
            }
        }
    }
    
    if let Err(e) = writer.close() {
        eprintln!("Erro ao fechar arquivo: {}", e);
        process::exit(1);
    }
    
    println!("✅ Tradução concluída com sucesso!");
}