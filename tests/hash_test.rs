use weap::cmds::hash::{HashCmd, HashAlgo};

#[test]
fn test_hash_md5() {
    let cmd = HashCmd {
        text: Some("hello".to_string()),
        algo: HashAlgo::Md5,
        file: None,
    };
    assert_eq!(cmd.compute().unwrap(), "5d41402abc4b2a76b9719d911017c592");
}

#[test]
fn test_hash_sha256() {
    let cmd = HashCmd {
        text: Some("hello".to_string()),
        algo: HashAlgo::Sha256,
        file: None,
    };
    assert_eq!(
        cmd.compute().unwrap(),
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
}

#[test]
fn test_hash_sha512() {
    let cmd = HashCmd {
        text: Some("hello".to_string()),
        algo: HashAlgo::Sha512,
        file: None,
    };
    assert_eq!(
        cmd.compute().unwrap(),
        "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"
    );
}

#[test]
fn test_hash_file() {
    let path = "/tmp/weap_test_hash_integration.txt";
    std::fs::write(path, "hello").unwrap();
    let cmd = HashCmd {
        text: None,
        algo: HashAlgo::Md5,
        file: Some(path.to_string()),
    };
    assert_eq!(cmd.compute().unwrap(), "5d41402abc4b2a76b9719d911017c592");
    std::fs::remove_file(path).unwrap();
}
