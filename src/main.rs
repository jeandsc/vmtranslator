mod parser;
mod code_writer;

use parser::Parser;
use code_writer::CodeWriter;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: vmtranslator <arquivo.vm ou diretorio>");
        process::exit(1);
    }

    let input = &args[1];
    let path = Path::new(input);

    // Coleta arquivos .vm e define nome do .asm de saída
    let (vm_files, output_name) = if path.is_dir() {
        let mut files: Vec<_> = fs::read_dir(path)
            .expect("Erro ao ler diretório")
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map_or(false, |ext| ext == "vm")
            })
            .map(|e| e.path())
            .collect();
        files.sort(); // ordem determinística

        let dir_name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        (files, format!("{}.asm", dir_name))
    } else {
        let filename = path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        (vec![path.to_path_buf()], format!("{}.asm", filename))
    };

    println!("Traduzindo {} arquivo(s) -> {}", vm_files.len(), output_name);

    // Cria o CodeWriter (já escreve o bootstrap no construtor)
    let mut writer = match CodeWriter::new(&output_name) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Erro ao criar arquivo {}: {}", output_name, e);
            process::exit(1);
        }
    };

    // Processa cada arquivo .vm
    for vm_path in vm_files {
        let module_name = vm_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        println!("  Processando: {}", vm_path.display());

        // Atualiza o nome do módulo (para variáveis static)
        writer.set_filename(&module_name);

        let mut parser = match Parser::new(vm_path.to_str().unwrap()) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Erro ao ler {}: {}", vm_path.display(), e);
                process::exit(1);
            }
        };

        while parser.has_more_commands() {
            parser.advance();
            let cmd = parser.current();

            let result = match cmd.cmd_type {
                parser::CommandType::CPush => {
                    writer.write_push(&cmd.arg1, cmd.arg2.unwrap())
                }
                parser::CommandType::CPop => {
                    writer.write_pop(&cmd.arg1, cmd.arg2.unwrap())
                }
                parser::CommandType::CArithmetic => {
                    writer.write_arithmetic(&cmd.arg1)
                }
                parser::CommandType::CLabel => writer.write_label(&cmd.arg1),
                parser::CommandType::CGoto => writer.write_goto(&cmd.arg1),
                parser::CommandType::CIf => writer.write_if(&cmd.arg1),
                parser::CommandType::CFunction => {
                    writer.write_function(&cmd.arg1, cmd.arg2.unwrap())
                }
                parser::CommandType::CCall => {
                    writer.write_call(&cmd.arg1, cmd.arg2.unwrap())
                }
                parser::CommandType::CReturn => writer.write_return(),
            };

            if let Err(e) = result {
                eprintln!("Erro ao escrever comando {:?}: {}", cmd.cmd_type, e);
                process::exit(1);
            }
        }
    }

    if let Err(e) = writer.close() {
        eprintln!("Erro ao fechar arquivo: {}", e);
        process::exit(1);
    }

    println!("✅ Tradução concluída: {}", output_name);
}