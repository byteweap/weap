use weap::cmds::SysCmd;

#[test]
fn test_sys_execute() {
    let cmd = SysCmd {};
    cmd.execute();
}
