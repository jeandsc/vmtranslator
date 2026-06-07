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
#[should_panic(expected = "Não pode pop constant")]
fn test_write_pop_constant() {
    let mut writer = CodeWriter::new("test_pop_constant.asm").unwrap();
    writer.write_pop("constant", 5).unwrap();
    std::fs::remove_file("test_pop_constant.asm").unwrap();
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