use clap::Parser;
use serde::Deserialize;
use std::fmt::Formatter;

#[derive(Deserialize)]
pub(crate) struct IpInfo {
    #[serde(rename = "query")]
    ip: String,
    #[serde(rename = "country")]
    country: String,
    #[serde(rename = "countryCode")]
    country_code: String,
    #[serde(rename = "region")]
    region: String,
    #[serde(rename = "regionName")]
    region_name: String,
    #[serde(rename = "city")]
    city: String,
    #[serde(rename = "zip")]
    zip: String,
    #[serde(rename = "lat")]
    lat: f64,
    #[serde(rename = "lon")]
    lon: f64,
    #[serde(rename = "timezone")]
    timezone: String,
    #[serde(rename = "isp")]
    isp: String,
    #[serde(rename = "org")]
    org: String,
    #[serde(rename = "as")]
    as_info: String,
}

impl std::fmt::Display for IpInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "🌐 IP信息")?;
        writeln!(f, "├─ 地址: {}", self.ip)?;
        writeln!(f, "├─ 位置: {},{}({}),{}", self.city, self.region_name, self.region, self.country)?;
        writeln!(f, "├─ 国家: {} ({})", self.country, self.country_code)?;
        writeln!(f, "├─ 时区: {}", self.timezone)?;
        writeln!(f, "├─ 组织: {}", self.org)?;
        writeln!(f, "├─ ISP: {}", self.isp)?;
        writeln!(f, "├─ ZIP: {}", self.zip)?;
        writeln!(f, "├─ AS: {}", self.as_info)?;
        write!(f, "└─ 坐标: {:.4},{:.4}", self.lat, self.lon)
    }
}

#[derive(Parser)]
pub struct IpCmd {
    /// 指定IP地址, 默认为本机IP
    pub ip: Option<String>,
}

impl IpCmd {
    pub(crate) fn build_url(&self) -> String {
        match &self.ip {
            Some(ip) if !ip.trim().is_empty() => format!("https://ip-api.com/json/{}", ip),
            _ => "https://ip-api.com/json/".to_string(),
        }
    }

    pub fn execute(&self) {
        let url = self.build_url();
        match reqwest::blocking::get(&url) {
            Ok(resp) => match resp.json::<IpInfo>() {
                Ok(info) => println!("{}", info),
                Err(e) => eprintln!("解析响应失败: {}", e),
            },
            Err(e) => eprintln!("请求失败: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_info() -> IpInfo {
        IpInfo {
            ip: "8.8.8.8".to_string(),
            country: "United States".to_string(),
            country_code: "US".to_string(),
            region: "CA".to_string(),
            region_name: "California".to_string(),
            city: "Mountain View".to_string(),
            zip: "94043".to_string(),
            lat: 37.4056,
            lon: -122.0775,
            timezone: "America/Los_Angeles".to_string(),
            isp: "Google LLC".to_string(),
            org: "Google LLC".to_string(),
            as_info: "AS15169 Google LLC".to_string(),
        }
    }

    #[test]
    fn test_build_url_with_ip() {
        let cmd = IpCmd {
            ip: Some("8.8.8.8".to_string()),
        };
        assert_eq!(cmd.build_url(), "https://ip-api.com/json/8.8.8.8");
    }

    #[test]
    fn test_build_url_with_none() {
        let cmd = IpCmd { ip: None };
        assert_eq!(cmd.build_url(), "https://ip-api.com/json/");
    }

    #[test]
    fn test_build_url_with_empty_string() {
        let cmd = IpCmd {
            ip: Some("".to_string()),
        };
        assert_eq!(cmd.build_url(), "https://ip-api.com/json/");
    }

    #[test]
    fn test_build_url_with_whitespace() {
        let cmd = IpCmd {
            ip: Some("   ".to_string()),
        };
        assert_eq!(cmd.build_url(), "https://ip-api.com/json/");
    }

    #[test]
    fn test_build_url_with_ipv6() {
        let cmd = IpCmd {
            ip: Some("2001:4860:4860::8888".to_string()),
        };
        assert_eq!(
            cmd.build_url(),
            "https://ip-api.com/json/2001:4860:4860::8888"
        );
    }

    #[test]
    fn test_display_ip_info() {
        let info = make_test_info();
        let output = format!("{}", info);
        assert!(output.contains("🌐 IP信息"));
        assert!(output.contains("8.8.8.8"));
        assert!(output.contains("United States"));
        assert!(output.contains("US"));
        assert!(output.contains("California"));
        assert!(output.contains("Mountain View"));
        assert!(output.contains("94043"));
        assert!(output.contains("America/Los_Angeles"));
        assert!(output.contains("Google LLC"));
        assert!(output.contains("AS15169 Google LLC"));
        assert!(output.contains("37.4056"));
        assert!(output.contains("-122.0775"));
        assert!(output.contains("└─ 坐标:"));
    }

    #[test]
    fn test_display_format_structure() {
        let info = make_test_info();
        let output = format!("{}", info);
        assert!(output.contains("├─ 地址:"));
        assert!(output.contains("├─ 位置:"));
        assert!(output.contains("├─ 国家:"));
        assert!(output.contains("├─ 时区:"));
        assert!(output.contains("├─ 组织:"));
        assert!(output.contains("├─ ISP:"));
        assert!(output.contains("├─ ZIP:"));
        assert!(output.contains("├─ AS:"));
        assert!(output.contains("└─ 坐标:"));
    }

    #[test]
    fn test_deserialize_ip_info() {
        let json = r#"{
            "query": "1.2.3.4",
            "country": "Japan",
            "countryCode": "JP",
            "region": "13",
            "regionName": "Tokyo",
            "city": "Tokyo",
            "zip": "100-0001",
            "lat": 35.6895,
            "lon": 139.6917,
            "timezone": "Asia/Tokyo",
            "isp": "Example ISP",
            "org": "Example Org",
            "as": "AS12345 Example"
        }"#;
        let info: IpInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.ip, "1.2.3.4");
        assert_eq!(info.country, "Japan");
        assert_eq!(info.country_code, "JP");
        assert_eq!(info.region, "13");
        assert_eq!(info.region_name, "Tokyo");
        assert_eq!(info.city, "Tokyo");
        assert_eq!(info.zip, "100-0001");
        assert_eq!(info.lat, 35.6895);
        assert_eq!(info.lon, 139.6917);
        assert_eq!(info.timezone, "Asia/Tokyo");
        assert_eq!(info.isp, "Example ISP");
        assert_eq!(info.org, "Example Org");
        assert_eq!(info.as_info, "AS12345 Example");
    }

    #[test]
    fn test_execute_with_ip() {
        let cmd = IpCmd {
            ip: Some("8.8.8.8".to_string()),
        };
        cmd.execute();
    }

    #[test]
    fn test_execute_without_ip() {
        let cmd = IpCmd { ip: None };
        cmd.execute();
    }
}
