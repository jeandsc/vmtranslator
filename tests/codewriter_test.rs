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