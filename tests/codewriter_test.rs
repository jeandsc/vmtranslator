use vmtranslator::CodeWriter;

#[test]
fn test_write_push_constant() {
    let mut writer = CodeWriter::new("test_push.asm").unwrap();
    writer.write_push("constant", 7).unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_push.asm").unwrap();
    let expected = "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
    
    assert_eq!(content, expected);
    
    std::fs::remove_file("test_push.asm").unwrap();
}

#[test]
fn test_write_push_local() {
    let mut writer = CodeWriter::new("test_local.asm").unwrap();
    writer.write_push("local", 3).unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_local.asm").unwrap();
    let expected = "@3\nD=A\n@LCL\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
    
    assert_eq!(content, expected);
    
    std::fs::remove_file("test_local.asm").unwrap();
}

#[test]
fn test_write_push_argument() {
    let mut writer = CodeWriter::new("test_arg.asm").unwrap();
    writer.write_push("argument", 5).unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_arg.asm").unwrap();
    let expected = "@5\nD=A\n@ARG\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_arg.asm").unwrap();
}

#[test]
fn test_write_push_this() {
    let mut writer = CodeWriter::new("test_this.asm").unwrap();
    writer.write_push("this", 2).unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_this.asm").unwrap();
    let expected = "@2\nD=A\n@THIS\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_this.asm").unwrap();
}

#[test]
fn test_write_push_that() {
    let mut writer = CodeWriter::new("test_that.asm").unwrap();
    writer.write_push("that", 9).unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_that.asm").unwrap();
    let expected = "@9\nD=A\n@THAT\nA=M+D\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_that.asm").unwrap();
}

#[test]
fn test_write_push_temp() {
    let mut writer = CodeWriter::new("test_temp.asm").unwrap();
    writer.write_push("temp", 1).unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_temp.asm").unwrap();
    let expected = "@6\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_temp.asm").unwrap();
}

#[test]
fn test_write_push_pointer() {
    let mut writer = CodeWriter::new("test_pointer.asm").unwrap();
    writer.write_push("pointer", 0).unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_pointer.asm").unwrap();
    let expected = "@3\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_pointer.asm").unwrap();
}

#[test]
fn test_write_push_static() {
    let mut writer = CodeWriter::new("test_static.asm").unwrap();
    writer.write_push("static", 42).unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_static.asm").unwrap();
    let expected = "@test_static.42\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_static.asm").unwrap();
}

#[test]
fn test_write_pop_local() {
    let mut writer = CodeWriter::new("test_pop_local.asm").unwrap();
    writer.write_pop("local", 3).unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_pop_local.asm").unwrap();
    let expected = "@3\nD=A\n@LCL\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_pop_local.asm").unwrap();
}
#[test]
fn test_write_pop_argument() {
    let mut writer = CodeWriter::new("test_pop_argument.asm").unwrap();
    writer.write_pop("argument", 5).unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_pop_argument.asm").unwrap();
    let expected = "@5\nD=A\n@ARG\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_pop_argument.asm").unwrap();
}

#[test]
fn test_write_pop_this() {
    let mut writer = CodeWriter::new("test_pop_this.asm").unwrap();
    writer.write_pop("this", 2).unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_pop_this.asm").unwrap();
    let expected = "@2\nD=A\n@THIS\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_pop_this.asm").unwrap();
}

#[test]
fn test_write_pop_that() {
    let mut writer = CodeWriter::new("test_pop_that.asm").unwrap();
    writer.write_pop("that", 9).unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_pop_that.asm").unwrap();
    let expected = "@9\nD=A\n@THAT\nD=M+D\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_pop_that.asm").unwrap();
}

#[test]
fn test_write_pop_temp() {
    let mut writer = CodeWriter::new("test_pop_temp.asm").unwrap();
    writer.write_pop("temp", 1).unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_pop_temp.asm").unwrap();
    let expected = "@6\nD=A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_pop_temp.asm").unwrap();
}

#[test]
fn test_write_pop_pointer() {
    let mut writer = CodeWriter::new("test_pop_pointer.asm").unwrap();
    writer.write_pop("pointer", 0).unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_pop_pointer.asm").unwrap();
    let expected = "@3\nD=A\n@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_pop_pointer.asm").unwrap();
}

#[test]
fn test_write_pop_static() {
    let mut writer = CodeWriter::new("test_pop_static.asm").unwrap();
    
    writer.write_pop("static", 42).unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_pop_static.asm").unwrap();
    let expected = "@SP\nAM=M-1\nD=M\n@test_pop_static.42\nM=D\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_pop_static.asm").unwrap();
}


#[test]
fn test_write_arithmetic_add() {
    let mut writer = CodeWriter::new("test_add.asm").unwrap();
    writer.write_arithmetic("add").unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_add.asm").unwrap();
    let expected = "@SP\nAM=M-1\nD=M\nA=A-1\nM=D+M\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_add.asm").unwrap();
}
#[test]
fn test_write_arithmetic_sub() {
    let mut writer = CodeWriter::new("test_sub.asm").unwrap();
    writer.write_arithmetic("sub").unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_sub.asm").unwrap();
    let expected = "@SP\nAM=M-1\nD=M\nA=A-1\nM=M-D\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_sub.asm").unwrap();
}
#[test]
fn test_write_arithmetic_neg() {
    let mut writer = CodeWriter::new("test_neg.asm").unwrap();
    writer.write_arithmetic("neg").unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_neg.asm").unwrap();
    let expected = "@SP\nA=M-1\nM=-M\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_neg.asm").unwrap();
}
#[test]
fn test_write_arithmetic_and() {
    let mut writer = CodeWriter::new("test_and.asm").unwrap();
    writer.write_arithmetic("and").unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_and.asm").unwrap();
    let expected = "@SP\nAM=M-1\nD=M\nA=A-1\nM=D&M\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_and.asm").unwrap();
}

#[test]
fn test_write_arithmetic_or() {
    let mut writer = CodeWriter::new("test_or.asm").unwrap();
    writer.write_arithmetic("or").unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_or.asm").unwrap();
    let expected = "@SP\nAM=M-1\nD=M\nA=A-1\nM=D|M\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_or.asm").unwrap();
}

#[test]
fn test_write_arithmetic_not() {
    let mut writer = CodeWriter::new("test_not.asm").unwrap();
    writer.write_arithmetic("not").unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_not.asm").unwrap();
    let expected = "@SP\nA=M-1\nM=!M\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_not.asm").unwrap();
}
#[test]
fn test_write_arithmetic_eq() {
    let mut writer = CodeWriter::new("test_eq.asm").unwrap();
    writer.write_arithmetic("eq").unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_eq.asm").unwrap();
    let expected = "@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@EQ_TRUE_0\nD;JEQ\nD=0\n@EQ_END_0\n0;JMP\n(EQ_TRUE_0)\nD=-1\n(EQ_END_0)\n@SP\nA=M-1\nM=D\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_eq.asm").unwrap();
}

#[test]
fn test_write_arithmetic_gt() {
    let mut writer = CodeWriter::new("test_gt.asm").unwrap();
    writer.write_arithmetic("gt").unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_gt.asm").unwrap();
    let expected = "@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@GT_TRUE_0\nD;JGT\nD=0\n@GT_END_0\n0;JMP\n(GT_TRUE_0)\nD=-1\n(GT_END_0)\n@SP\nA=M-1\nM=D\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_gt.asm").unwrap();
}

#[test]
fn test_write_arithmetic_lt() {
    let mut writer = CodeWriter::new("test_lt.asm").unwrap();
    writer.write_arithmetic("lt").unwrap();
    writer.close().unwrap();
    
    let content = std::fs::read_to_string("test_lt.asm").unwrap();
    let expected = "@SP\nAM=M-1\nD=M\nA=A-1\nD=M-D\n@LT_TRUE_0\nD;JLT\nD=0\n@LT_END_0\n0;JMP\n(LT_TRUE_0)\nD=-1\n(LT_END_0)\n@SP\nA=M-1\nM=D\n";
    
    assert_eq!(content, expected);
    std::fs::remove_file("test_lt.asm").unwrap();
}
#[test]
fn test_write_label() {
    let output_path = "temp_label.asm";
    let mut cw = CodeWriter::new(output_path).unwrap();
    
    cw.write_label("LOOP").unwrap();
    cw.close().unwrap();
    
    let content = std::fs::read_to_string(output_path).unwrap();
    // func_name está vazio, então sai ($LOOP)
    assert!(content.contains("($LOOP)"));
    
    std::fs::remove_file(output_path).unwrap();
}
#[test]
fn test_write_goto() {
    let output_path = "temp_goto.asm";
    let mut cw = CodeWriter::new(output_path).unwrap();
    
    cw.write_goto("LOOP").unwrap();
    cw.close().unwrap();
    
    let content = std::fs::read_to_string(output_path).unwrap();
    // func_name está vazio, então sai @$LOOP
    assert!(content.contains("@$LOOP"));
    assert!(content.contains("0;JMP"));
    
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn test_write_goto_multiple() {
    let output_path = "temp_gotos.asm";
    let mut cw = CodeWriter::new(output_path).unwrap();
    
    cw.write_goto("START").unwrap();
    cw.write_goto("END").unwrap();
    cw.close().unwrap();
    
    let content = std::fs::read_to_string(output_path).unwrap();
    assert!(content.contains("@$START"));
    assert!(content.contains("@$END"));
    assert!(content.contains("0;JMP"));
    
    std::fs::remove_file(output_path).unwrap();
}
#[test]
fn test_write_if() {
    let output_path = "temp_if.asm";
    let mut cw = CodeWriter::new(output_path).unwrap();
    
    cw.write_if("END").unwrap();
    cw.close().unwrap();
    
    let content = std::fs::read_to_string(output_path).unwrap();
    // func_name está vazio, então sai @$END
    assert!(content.contains("@SP"));
    assert!(content.contains("AM=M-1"));
    assert!(content.contains("D=M"));
    assert!(content.contains("@$END"));
    assert!(content.contains("D;JNE"));
    
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn test_write_if_multiple() {
    let output_path = "temp_ifs.asm";
    let mut cw = CodeWriter::new(output_path).unwrap();
    
    cw.write_if("LOOP").unwrap();
    cw.write_if("EXIT").unwrap();
    cw.close().unwrap();
    
    let content = std::fs::read_to_string(output_path).unwrap();
    assert!(content.contains("@$LOOP"));
    assert!(content.contains("@$EXIT"));
    // D;JNE deve aparecer 2 vezes
    let jne_count = content.matches("D;JNE").count();
    assert_eq!(jne_count, 2);
    
    std::fs::remove_file(output_path).unwrap();
}
#[test]
fn test_write_function_no_locals() {
    let output_path = "temp_func_nolocals.asm";
    let mut cw = CodeWriter::new(output_path).unwrap();
    
    cw.write_function("Sys.init", 0).unwrap();
    cw.close().unwrap();
    
    let content = std::fs::read_to_string(output_path).unwrap();
    assert!(content.contains("(Sys.init)"));
    
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn test_write_function_with_locals() {
    let output_path = "temp_func_locals.asm";
    let mut cw = CodeWriter::new(output_path).unwrap();
    
    cw.write_function("Sys.main", 2).unwrap();
    cw.close().unwrap();
    
    let content = std::fs::read_to_string(output_path).unwrap();
    assert!(content.contains("(Sys.main)"));

    let push_count = content.matches("@0\nD=A").count();
    assert_eq!(push_count, 2, "Esperava 2 inicializações de local");
    
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn test_write_function_updates_context() {
    let output_path = "temp_func_context.asm";
    let mut cw = CodeWriter::new(output_path).unwrap();
    
    cw.write_function("Foo", 0).unwrap();
    cw.write_label("LOOP").unwrap();
    cw.close().unwrap();
    
    let content = std::fs::read_to_string(output_path).unwrap();
    assert!(content.contains("(Foo)"));
    assert!(content.contains("(Foo$LOOP)"));
    
    std::fs::remove_file(output_path).unwrap();
}
#[test]
fn test_write_call_basic() {
    let output_path = "temp_call.asm";
    let mut cw = CodeWriter::new(output_path).unwrap();
    
    cw.write_call("Sys.main", 0).unwrap();
    cw.close().unwrap();
    
    let content = std::fs::read_to_string(output_path).unwrap();

    assert!(content.contains("(Sys.main$ret.0)"));

    assert!(content.contains("@Sys.main"));
    assert!(content.contains("0;JMP"));

    assert!(content.contains("@ARG"));
    assert!(content.contains("@LCL"));

    assert!(content.contains("@LCL"));
    assert!(content.contains("@ARG"));
    assert!(content.contains("@THIS"));
    assert!(content.contains("@THAT"));
    
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn test_write_call_increments_counter() {
    let output_path = "temp_calls.asm";
    let mut cw = CodeWriter::new(output_path).unwrap();
    
    cw.write_call("foo", 2).unwrap();
    cw.write_call("bar", 1).unwrap();
    cw.close().unwrap();
    
    let content = std::fs::read_to_string(output_path).unwrap();
    
    assert!(content.contains("(foo$ret.0)"));
    assert!(content.contains("(bar$ret.1)"));
    
    std::fs::remove_file(output_path).unwrap();
}
#[test]
fn test_write_return_basic() {
    let output_path = "temp_return.asm";
    let mut cw = CodeWriter::new(output_path).unwrap();
    
    cw.write_return().unwrap();
    cw.close().unwrap();
    
    let content = std::fs::read_to_string(output_path).unwrap();
    
    
    assert!(content.contains("@LCL"));
    assert!(content.contains("@R13"));
    
    
    assert!(content.contains("@5"));
    assert!(content.contains("@R14"));
    
    
    assert!(content.contains("@SP"));
    assert!(content.contains("AM=M-1"));
    assert!(content.contains("@ARG"));
    

    assert!(content.contains("M=D+1"));
    

    assert!(content.contains("@THAT"));
    assert!(content.contains("@THIS"));
    
    
    assert!(content.contains("0;JMP"));
    
    std::fs::remove_file(output_path).unwrap();
}

#[test]
fn test_write_return_multiple() {
    let output_path = "temp_returns.asm";
    let mut cw = CodeWriter::new(output_path).unwrap();
    
    cw.write_return().unwrap();
    cw.write_return().unwrap();
    cw.close().unwrap();
    
    let content = std::fs::read_to_string(output_path).unwrap();
    
    
    let lcl_count = content.matches("@LCL").count();
    assert!(lcl_count >= 4, "Esperava múltiplas referências a LCL, encontrou {}", lcl_count);
    
    std::fs::remove_file(output_path).unwrap();
}