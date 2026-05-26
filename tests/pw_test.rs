use weap::cmds::PwCmd;

#[test]
fn test_pw_default_length() {
    let cmd = PwCmd {
        length: 16,
        special: false,
    };
    let pw = cmd.generate();
    assert_eq!(pw.len(), 16);
    assert!(pw.chars().all(|c| c.is_ascii_alphanumeric()));
}

#[test]
fn test_pw_custom_length() {
    let cmd = PwCmd {
        length: 32,
        special: false,
    };
    let pw = cmd.generate();
    assert_eq!(pw.len(), 32);
}

#[test]
fn test_pw_length_1() {
    let cmd = PwCmd {
        length: 1,
        special: false,
    };
    let pw = cmd.generate();
    assert_eq!(pw.len(), 1);
}

#[test]
fn test_pw_length_0() {
    let cmd = PwCmd {
        length: 0,
        special: false,
    };
    let pw = cmd.generate();
    assert!(pw.is_empty());
}

#[test]
fn test_pw_with_special_chars() {
    let cmd = PwCmd {
        length: 100,
        special: true,
    };
    let pw = cmd.generate();
    assert_eq!(pw.len(), 100);
    let has_special = pw.chars().any(|c| !c.is_ascii_alphanumeric());
    assert!(has_special, "密码应包含特殊字符");
}

#[test]
fn test_pw_without_special_chars() {
    let cmd = PwCmd {
        length: 100,
        special: false,
    };
    let pw = cmd.generate();
    assert_eq!(pw.len(), 100);
    assert!(pw.chars().all(|c| c.is_ascii_alphanumeric()));
}

#[test]
fn test_pw_uniqueness() {
    let cmd = PwCmd {
        length: 32,
        special: false,
    };
    let pw1 = cmd.generate();
    let pw2 = cmd.generate();
    assert_ne!(pw1, pw2);
}

#[test]
fn test_pw_execute_default() {
    let cmd = PwCmd {
        length: 16,
        special: false,
    };
    cmd.execute();
}

#[test]
fn test_pw_execute_with_special() {
    let cmd = PwCmd {
        length: 20,
        special: true,
    };
    cmd.execute();
}
