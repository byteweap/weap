use weap::cmds::UuidCmd;

#[test]
fn test_uuid_with_hyphens() {
    let cmd = UuidCmd {
        no_hyphens: false,
    };
    let uuid = cmd.generate();
    assert_eq!(uuid.len(), 36);
    assert_eq!(uuid.chars().filter(|c| *c == '-').count(), 4);
    assert!(uuid.chars().all(|c| c.is_ascii_hexdigit() || c == '-'));
}

#[test]
fn test_uuid_no_hyphens() {
    let cmd = UuidCmd {
        no_hyphens: true,
    };
    let uuid = cmd.generate();
    assert_eq!(uuid.len(), 32);
    assert!(!uuid.contains('-'));
    assert!(uuid.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_uuid_uniqueness() {
    let cmd = UuidCmd {
        no_hyphens: false,
    };
    let uuid1 = cmd.generate();
    let uuid2 = cmd.generate();
    assert_ne!(uuid1, uuid2);
}

#[test]
fn test_uuid_no_hyphens_uniqueness() {
    let cmd = UuidCmd {
        no_hyphens: true,
    };
    let uuid1 = cmd.generate();
    let uuid2 = cmd.generate();
    assert_ne!(uuid1, uuid2);
}

#[test]
fn test_uuid_execute_with_hyphens() {
    let cmd = UuidCmd {
        no_hyphens: false,
    };
    cmd.execute();
}

#[test]
fn test_uuid_execute_no_hyphens() {
    let cmd = UuidCmd {
        no_hyphens: true,
    };
    cmd.execute();
}
