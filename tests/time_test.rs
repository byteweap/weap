use weap::cmds::time::TimeCmd;

#[test]
fn test_time_default() {
    let cmd = TimeCmd {
        format: "%Y-%m-%d %H:%M:%S".to_string(),
        unix: false,
        timezone: None,
        countdown: None,
    };
    cmd.execute();
}

#[test]
fn test_time_with_unix() {
    let cmd = TimeCmd {
        format: "%Y-%m-%d %H:%M:%S".to_string(),
        unix: true,
        timezone: None,
        countdown: None,
    };
    cmd.execute();
}

#[test]
fn test_time_with_valid_timezone() {
    let cmd = TimeCmd {
        format: "%Y-%m-%d %H:%M:%S".to_string(),
        unix: false,
        timezone: Some("Asia/Shanghai".to_string()),
        countdown: None,
    };
    cmd.execute();
}

#[test]
fn test_time_with_invalid_timezone() {
    let cmd = TimeCmd {
        format: "%Y-%m-%d %H:%M:%S".to_string(),
        unix: false,
        timezone: Some("Invalid/Timezone".to_string()),
        countdown: None,
    };
    cmd.execute();
}

#[test]
fn test_time_with_all_options() {
    let cmd = TimeCmd {
        format: "%H:%M:%S".to_string(),
        unix: true,
        timezone: Some("America/New_York".to_string()),
        countdown: None,
    };
    cmd.execute();
}

#[test]
fn test_time_with_countdown() {
    let cmd = TimeCmd {
        format: "%Y-%m-%d %H:%M:%S".to_string(),
        unix: false,
        timezone: None,
        countdown: Some(1),
    };
    cmd.execute();
}

#[test]
fn test_time_custom_format() {
    let cmd = TimeCmd {
        format: "%H:%M".to_string(),
        unix: false,
        timezone: None,
        countdown: None,
    };
    cmd.execute();
}

#[test]
fn test_time_with_tokyo() {
    let cmd = TimeCmd {
        format: "%Y-%m-%d %H:%M:%S".to_string(),
        unix: false,
        timezone: Some("Asia/Tokyo".to_string()),
        countdown: None,
    };
    cmd.execute();
}
