use weap::cmds::b64::{B64Cmd, B64Action};

#[test]
fn test_b64_encode() {
    let cmd = B64Cmd {
        action: B64Action::Encode,
        text: "hello".to_string(),
    };
    assert_eq!(cmd.process().unwrap(), "aGVsbG8=");
}

#[test]
fn test_b64_decode() {
    let cmd = B64Cmd {
        action: B64Action::Decode,
        text: "aGVsbG8=".to_string(),
    };
    assert_eq!(cmd.process().unwrap(), "hello");
}

#[test]
fn test_b64_roundtrip() {
    let original = "Hello, World! 你好世界";
    let cmd = B64Cmd {
        action: B64Action::Encode,
        text: original.to_string(),
    };
    let encoded = cmd.process().unwrap();
    let decode_cmd = B64Cmd {
        action: B64Action::Decode,
        text: encoded,
    };
    assert_eq!(decode_cmd.process().unwrap(), original);
}

#[test]
fn test_b64_decode_invalid() {
    let cmd = B64Cmd {
        action: B64Action::Decode,
        text: "!!!invalid!!!".to_string(),
    };
    assert!(cmd.process().is_err());
}
