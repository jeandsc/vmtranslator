# VMTranslator - Parte 1

## Integrantes
- Jeanderson da Silva Campos: 20250013677

## Linguagem
- Rust

## Instruções de build e execução

Build:
cargo build --release

Executar:
cargo run arquivo.vm

Exemplo:
cargo run arquivos/SimpleAdd.vm

Testes:
cargo test --test parser_test
cargo test --test codewriter_test

## Justificativa
Rust foi escolhida por segurança de memória, performance e pattern matching.

## Status
✅ SimpleAdd.vm
✅ BasicTest.vm