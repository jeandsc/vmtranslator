use vmtranslator::{Parser, CommandType};

#[test]
fn test_push_constant() {
    let content = "push constant 7";
    std::fs::write("temp_constant.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_constant.vm").unwrap();
    
    assert!(parser.has_more_commands());
    parser.advance();
    let cmd = parser.current();
    
    assert!(matches!(cmd.cmd_type, CommandType::CPush));
    assert_eq!(cmd.arg1, "constant");
    assert_eq!(cmd.arg2, Some(7));
    
    std::fs::remove_file("temp_constant.vm").unwrap();
}
#[test]
fn test_multiple_commands() {
    let content = "
        push constant 7
        push constant 8
        push constant 42
    ";
    std::fs::write("temp_commands.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_commands.vm").unwrap();
    
    let expected = vec![7, 8, 42];
    
    for &val in expected.iter() {
        assert!(parser.has_more_commands());
        parser.advance();
        let cmd = parser.current();
        assert_eq!(cmd.arg2, Some(val));
    }
    
    std::fs::remove_file("temp_commands.vm").unwrap();
}

#[test]
#[should_panic(expected = "Segmento inválido: xyz")]
fn test_invalid_segment() {
    let content = "push xyz 5";
    std::fs::write("temp_segment.vm", content).unwrap();
    let _ = Parser::new("temp_segment.vm").unwrap();
    std::fs::remove_file("temp_segment.vm").unwrap();
}

#[test]
#[should_panic(expected = "Comando desconhecido: foo")]
fn test_unknown_command() {
    let content = "foo bar 5";
    std::fs::write("temp_unknown.vm", content).unwrap();
    let _ = Parser::new("temp_unknown.vm").unwrap();
    std::fs::remove_file("temp_unknown.vm").unwrap();
}


#[test]
fn test_pop_local() {
    let content = "pop local 3";
    std::fs::write("temp_local.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_local.vm").unwrap();
    parser.advance();
    let cmd = parser.current();
    
    assert!(matches!(cmd.cmd_type, CommandType::CPop));
    assert_eq!(cmd.arg1, "local");
    assert_eq!(cmd.arg2, Some(3));
    
    std::fs::remove_file("temp_local.vm").unwrap();
}

#[test]
fn test_push_pop_mixed() {
    let content = "
        push constant 10
        pop local 0
        push constant 21
        pop argument 2
    ";
    std::fs::write("temp_mixed.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_mixed.vm").unwrap();
    
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
    
    std::fs::remove_file("temp_mixed.vm").unwrap();
}

#[test]
fn test_arithmetic_add() {
    let content = "add";
    std::fs::write("tempadd.vm", content).unwrap();
    
    let mut parser = Parser::new("tempadd.vm").unwrap();
    parser.advance();
    let cmd = parser.current();
    
    assert!(matches!(cmd.cmd_type, CommandType::CArithmetic));
    assert_eq!(cmd.arg1, "add");
    assert_eq!(cmd.arg2, None);
    
    std::fs::remove_file("tempadd.vm").unwrap();
}

#[test]
fn test_arithmetic_all() {
    let operations = vec!["add", "sub", "neg", "eq", "gt", "lt", "and", "or", "not"];
    
    for op in operations {
        let content = format!("{}", op);
        std::fs::write("temp_all.vm", &content).unwrap();
        
        let mut parser = Parser::new("temp_all.vm").unwrap();
        parser.advance();
        let cmd = parser.current();
        
        assert!(matches!(cmd.cmd_type, CommandType::CArithmetic));
        assert_eq!(cmd.arg1, op);
        assert_eq!(cmd.arg2, None);
        
        std::fs::remove_file("temp_all.vm").unwrap();
    }
}

#[test]
fn test_mixed_with_arithmetic() {
    let content = "
        push constant 7
        push constant 8
        add
        pop local 0
        push constant 5
        neg
        pop argument 1
    ";
    std::fs::write("temp_arithmetic.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_arithmetic.vm").unwrap();
    
    parser.advance();
    assert!(matches!(parser.current().cmd_type, CommandType::CPush));
    
    parser.advance();
    assert!(matches!(parser.current().cmd_type, CommandType::CPush));
    
    parser.advance();
    assert!(matches!(parser.current().cmd_type, CommandType::CArithmetic));
    assert_eq!(parser.current().arg1, "add");
    
    parser.advance();
    assert!(matches!(parser.current().cmd_type, CommandType::CPop));
    
    parser.advance();
    assert!(matches!(parser.current().cmd_type, CommandType::CPush));
    
    parser.advance();
    assert!(matches!(parser.current().cmd_type, CommandType::CArithmetic));
    assert_eq!(parser.current().arg1, "neg");
    
    parser.advance();
    assert!(matches!(parser.current().cmd_type, CommandType::CPop));
    
    std::fs::remove_file("temp_arithmetic.vm").unwrap();
}
#[test]
fn test_label() {
    let content = "label LOOP";
    std::fs::write("temp_label.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_label.vm").unwrap();
    parser.advance();
    let cmd = parser.current();
    
    assert!(matches!(cmd.cmd_type, CommandType::CLabel));
    assert_eq!(cmd.arg1, "LOOP");
    assert_eq!(cmd.arg2, None);
    
    std::fs::remove_file("temp_label.vm").unwrap();
}
#[test]
fn test_label_multiple() {
    let content = "
        label LOOP
        label END
        label WHILE
    ";
    std::fs::write("temp_labels.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_labels.vm").unwrap();
    
    let expected = vec!["LOOP", "END", "WHILE"];
    
    for &name in expected.iter() {
        assert!(parser.has_more_commands());
        parser.advance();
        let cmd = parser.current();
        assert!(matches!(cmd.cmd_type, CommandType::CLabel));
        assert_eq!(cmd.arg1, name);
        assert_eq!(cmd.arg2, None);
    }
    
    std::fs::remove_file("temp_labels.vm").unwrap();
}
#[test]
fn test_goto() {
    let content = "goto LOOP";
    std::fs::write("temp_goto.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_goto.vm").unwrap();
    parser.advance();
    let cmd = parser.current();
    
    assert!(matches!(cmd.cmd_type, CommandType::CGoto));
    assert_eq!(cmd.arg1, "LOOP");
    assert_eq!(cmd.arg2, None);
    
    std::fs::remove_file("temp_goto.vm").unwrap();
}

#[test]
fn test_goto_multiple() {
    let content = "
        goto START
        goto END
        goto LOOP
    ";
    std::fs::write("temp_gotos.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_gotos.vm").unwrap();
    
    let expected = vec!["START", "END", "LOOP"];
    
    for &name in expected.iter() {
        assert!(parser.has_more_commands());
        parser.advance();
        let cmd = parser.current();
        assert!(matches!(cmd.cmd_type, CommandType::CGoto));
        assert_eq!(cmd.arg1, name);
        assert_eq!(cmd.arg2, None);
    }
    
    std::fs::remove_file("temp_gotos.vm").unwrap();
}
#[test]
fn test_if_goto() {
    let content = "if-goto LOOP";
    std::fs::write("temp_ifgoto.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_ifgoto.vm").unwrap();
    parser.advance();
    let cmd = parser.current();
    
    assert!(matches!(cmd.cmd_type, CommandType::CIf));
    assert_eq!(cmd.arg1, "LOOP");
    assert_eq!(cmd.arg2, None);
    
    std::fs::remove_file("temp_ifgoto.vm").unwrap();
}

#[test]
fn test_if_goto_multiple() {
    let content = "
        if-goto END
        if-goto LOOP
        if-goto WHILE
    ";
    std::fs::write("temp_ifgotos.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_ifgotos.vm").unwrap();
    
    let expected = vec!["END", "LOOP", "WHILE"];
    
    for &name in expected.iter() {
        assert!(parser.has_more_commands());
        parser.advance();
        let cmd = parser.current();
        assert!(matches!(cmd.cmd_type, CommandType::CIf));
        assert_eq!(cmd.arg1, name);
        assert_eq!(cmd.arg2, None);
    }
    
    std::fs::remove_file("temp_ifgotos.vm").unwrap();
}
#[test]
fn test_function() {
    let content = "function Sys.init 0";
    std::fs::write("temp_function.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_function.vm").unwrap();
    parser.advance();
    let cmd = parser.current();
    
    assert!(matches!(cmd.cmd_type, CommandType::CFunction));
    assert_eq!(cmd.arg1, "Sys.init");
    assert_eq!(cmd.arg2, Some(0));
    
    std::fs::remove_file("temp_function.vm").unwrap();
}

#[test]
fn test_function_with_locals() {
    let content = "function Sys.main 2";
    std::fs::write("temp_function_locals.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_function_locals.vm").unwrap();
    parser.advance();
    let cmd = parser.current();
    
    assert!(matches!(cmd.cmd_type, CommandType::CFunction));
    assert_eq!(cmd.arg1, "Sys.main");
    assert_eq!(cmd.arg2, Some(2));
    
    std::fs::remove_file("temp_function_locals.vm").unwrap();
}

#[test]
fn test_function_multiple() {
    let content = "
        function Sys.init 0
        function Sys.main 2
        function Sys.add12 0
    ";
    std::fs::write("temp_functions.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_functions.vm").unwrap();
    
    let expected = vec![
        ("Sys.init", 0),
        ("Sys.main", 2),
        ("Sys.add12", 0),
    ];
    
    for &(name, nlocals) in expected.iter() {
        assert!(parser.has_more_commands());
        parser.advance();
        let cmd = parser.current();
        assert!(matches!(cmd.cmd_type, CommandType::CFunction));
        assert_eq!(cmd.arg1, name);
        assert_eq!(cmd.arg2, Some(nlocals));
    }
    
    std::fs::remove_file("temp_functions.vm").unwrap();
}
#[test]
fn test_call() {
    let content = "call Sys.main 0";
    std::fs::write("temp_call.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_call.vm").unwrap();
    parser.advance();
    let cmd = parser.current();
    
    assert!(matches!(cmd.cmd_type, CommandType::CCall));
    assert_eq!(cmd.arg1, "Sys.main");
    assert_eq!(cmd.arg2, Some(0));
    
    std::fs::remove_file("temp_call.vm").unwrap();
}

#[test]
fn test_call_with_args() {
    let content = "call Math.add 2";
    std::fs::write("temp_call_args.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_call_args.vm").unwrap();
    parser.advance();
    let cmd = parser.current();
    
    assert!(matches!(cmd.cmd_type, CommandType::CCall));
    assert_eq!(cmd.arg1, "Math.add");
    assert_eq!(cmd.arg2, Some(2));
    
    std::fs::remove_file("temp_call_args.vm").unwrap();
}

#[test]
fn test_call_multiple() {
    let content = "
        call Sys.main 0
        call Math.add 2
        call Sys.add12 1
    ";
    std::fs::write("temp_calls.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_calls.vm").unwrap();
    
    let expected = vec![
        ("Sys.main", 0),
        ("Math.add", 2),
        ("Sys.add12", 1),
    ];
    
    for &(name, nargs) in expected.iter() {
        assert!(parser.has_more_commands());
        parser.advance();
        let cmd = parser.current();
        assert!(matches!(cmd.cmd_type, CommandType::CCall));
        assert_eq!(cmd.arg1, name);
        assert_eq!(cmd.arg2, Some(nargs));
    }
    
    std::fs::remove_file("temp_calls.vm").unwrap();
}
#[test]
fn test_return() {
    let content = "return";
    std::fs::write("temp_return.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_return.vm").unwrap();
    parser.advance();
    let cmd = parser.current();
    
    assert!(matches!(cmd.cmd_type, CommandType::CReturn));
    assert_eq!(cmd.arg1, "");
    assert_eq!(cmd.arg2, None);
    
    std::fs::remove_file("temp_return.vm").unwrap();
}

#[test]
fn test_return_multiple() {
    let content = "
        return
        return
        return
    ";
    std::fs::write("temp_returns.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_returns.vm").unwrap();
    
    for _ in 0..3 {
        assert!(parser.has_more_commands());
        parser.advance();
        let cmd = parser.current();
        assert!(matches!(cmd.cmd_type, CommandType::CReturn));
        assert_eq!(cmd.arg1, "");
        assert_eq!(cmd.arg2, None);
    }
    
    std::fs::remove_file("temp_returns.vm").unwrap();
}

#[test]
fn test_return_mixed_with_function() {
    let content = "
        function Sys.init 0
        call Sys.main 0
        return
    ";
    std::fs::write("temp_mixed_fn.vm", content).unwrap();
    
    let mut parser = Parser::new("temp_mixed_fn.vm").unwrap();
    
    parser.advance();
    assert!(matches!(parser.current().cmd_type, CommandType::CFunction));
    assert_eq!(parser.current().arg1, "Sys.init");
    
    parser.advance();
    assert!(matches!(parser.current().cmd_type, CommandType::CCall));
    assert_eq!(parser.current().arg1, "Sys.main");
    
    parser.advance();
    assert!(matches!(parser.current().cmd_type, CommandType::CReturn));
    
    std::fs::remove_file("temp_mixed_fn.vm").unwrap();
}