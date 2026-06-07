use vmtranslator::{Parser, CommandType};

#[test]
fn test_push_constant() {
    let content = "push constant 7";
    std::fs::write("temp.vm", content).unwrap();
    
    let mut parser = Parser::new("temp.vm").unwrap();
    
    assert!(parser.has_more_commands());
    parser.advance();
    let cmd = parser.current();
    
    assert!(matches!(cmd.cmd_type, CommandType::CPush));
    assert_eq!(cmd.arg1, "constant");
    assert_eq!(cmd.arg2, Some(7));
    
    std::fs::remove_file("temp.vm").unwrap();
}
#[test]
fn test_multiple_commands() {
    let content = "
        push constant 7
        push constant 8
        push constant 42
    ";
    std::fs::write("temp.vm", content).unwrap();
    
    let mut parser = Parser::new("temp.vm").unwrap();
    
    let expected = vec![7, 8, 42];
    
    for &val in expected.iter() {
        assert!(parser.has_more_commands());
        parser.advance();
        let cmd = parser.current();
        assert_eq!(cmd.arg2, Some(val));
    }
    
    std::fs::remove_file("temp.vm").unwrap();
}

#[test]
#[should_panic(expected = "Segmento inválido: xyz")]
fn test_invalid_segment() {
    let content = "push xyz 5";
    std::fs::write("temp.vm", content).unwrap();
    let _ = Parser::new("temp.vm").unwrap();
    std::fs::remove_file("temp.vm").unwrap();
}

#[test]
#[should_panic(expected = "Comando desconhecido: foo")]
fn test_unknown_command() {
    let content = "foo bar 5";
    std::fs::write("temp.vm", content).unwrap();
    let _ = Parser::new("temp.vm").unwrap();
    std::fs::remove_file("temp.vm").unwrap();
}


#[test]
fn test_pop_local() {
    let content = "pop local 3";
    std::fs::write("temp.vm", content).unwrap();
    
    let mut parser = Parser::new("temp.vm").unwrap();
    parser.advance();
    let cmd = parser.current();
    
    assert!(matches!(cmd.cmd_type, CommandType::CPop));
    assert_eq!(cmd.arg1, "local");
    assert_eq!(cmd.arg2, Some(3));
    
    std::fs::remove_file("temp.vm").unwrap();
}

#[test]
fn test_push_pop_mixed() {
    let content = "
        push constant 10
        pop local 0
        push constant 21
        pop argument 2
    ";
    std::fs::write("temp.vm", content).unwrap();
    
    let mut parser = Parser::new("temp.vm").unwrap();
    
    parser.advance();
    assert!(matches!(parser.current().cmd_type, CommandType::CPush));
    assert_eq!(parser.current().arg2, Some(10));
    
    parser.advance();
    assert!(matches!(parser.current().cmd_type, CommandType::CPop));
    assert_eq!(parser.current().arg1, "local");
    
    parser.advance();
    assert!(matches!(parser.current().cmd_type, CommandType::CPush));
    assert_eq!(parser.current().arg2, Some(21));
    
    parser.advance();
    assert!(matches!(parser.current().cmd_type, CommandType::CPop));
    assert_eq!(parser.current().arg1, "argument");
    
    std::fs::remove_file("temp.vm").unwrap();
}