use weap::cmds::ip::IpCmd;

#[test]
fn test_ip_basic() {
    let cmd = IpCmd {
        ip: "8.8.8.8".to_string(),
    };
    cmd.execute();
}