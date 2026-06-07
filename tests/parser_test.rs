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
