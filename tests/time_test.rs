use weap::cmds::time::TimeCmd;
#[test]
fn test_time_basic() {
    let cmd = TimeCmd {
        format: "%Y-%m-%d %H:%M:%S".to_string(),
        unix: false,
        timezone: None,
        countdown: None,
    };
    cmd.execute();
}