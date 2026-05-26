use weap::cmds::ip::IpCmd;

#[test]
fn test_ip_with_address() {
    let cmd = IpCmd {
        ip: Some("8.8.8.8".to_string()),
    };
    cmd.execute();
}

#[test]
fn test_ip_default() {
    let cmd = IpCmd { ip: None };
    cmd.execute();
}

#[test]
fn test_ip_with_empty_string() {
    let cmd = IpCmd {
        ip: Some("".to_string()),
    };
    cmd.execute();
}

#[test]
fn test_ip_with_whitespace() {
    let cmd = IpCmd {
        ip: Some("   ".to_string()),
    };
    cmd.execute();
}
