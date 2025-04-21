use weap::cmds::UuidCmd;

#[test]
fn test_sys_basic() {
    let cmd = UuidCmd {
        no_hyphens: false,
    };
    cmd.execute();
}