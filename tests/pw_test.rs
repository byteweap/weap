use weap::cmds::PwCmd;
#[test]
fn test_pw_basic() {
    let cmd = PwCmd {
        length: 16,
    };
    cmd.execute();
}