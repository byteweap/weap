use clap::Parser;
use serde::Deserialize;
use std::fmt::Formatter;

#[derive(Deserialize)]
struct IpInfo {
    #[serde(rename = "query")]
    ip: String,
    // #[serde(rename = "status")]
    // status: String,
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
    ass: String,
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
        writeln!(f, "├─ AS: {}", self.ass)?;
        write!(f, "└─ 坐标: {:.4},{:.4}", self.lat, self.lon)
    }
}

#[derive(Parser)]
pub struct IpCmd {
    /// 指定IP地址, 默认为本机IP
    ip: Option<String>,
}

impl IpCmd {
    /// 执行IP地址命令
    pub fn execute(&self) {
        let url = match &self.ip {
            Some(ip) => format!("http://ip-api.com/json/{}", ip),
            None => "http://ip-api.com/json/".to_string(),
        };
        let resp = reqwest::blocking::get(url)
            .expect("请求失败")
            .json::<IpInfo>()
            .expect("解析响应失败");
        println!("{}", resp);
    }
}
