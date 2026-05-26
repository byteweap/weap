use weap::cmds::base::BaseCmd;

#[test]
fn test_base_255() {
    let cmd = BaseCmd {
        value: "255".to_string(),
        from: 10,
    };
    let result = cmd.convert().unwrap();
    assert!(result[0].contains("11111111"));
    assert!(result[1].contains("377"));
    assert!(result[2].contains("255"));
    assert!(result[3].contains("ff"));
}

#[test]
fn test_base_from_binary() {
    let cmd = BaseCmd {
        value: "11111111".to_string(),
        from: 2,
    };
    let result = cmd.convert().unwrap();
    assert!(result[2].contains("255"));
}

#[test]
fn test_base_from_hex() {
    let cmd = BaseCmd {
        value: "ff".to_string(),
        from: 16,
    };
    let result = cmd.convert().unwrap();
    assert!(result[2].contains("255"));
}

#[test]
fn test_base_zero() {
    let cmd = BaseCmd {
        value: "0".to_string(),
        from: 10,
    };
    let result = cmd.convert().unwrap();
    assert!(result[0].contains("0"));
    assert!(result[3].contains("0"));
}

#[test]
fn test_base_invalid() {
    let cmd = BaseCmd {
        value: "xyz".to_string(),
        from: 10,
    };
    assert!(cmd.convert().is_err());
}
