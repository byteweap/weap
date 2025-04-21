use weap::cmds::SysCmd;

#[test]
fn test_sys_basic() {
    let cmd = SysCmd {};
    cmd.execute();
}