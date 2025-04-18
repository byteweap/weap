# weap

一个简单实用的命令行小工具

## 功能特性

- UUID 生成
  - 支持带连字符的 UUID 生成
  - 支持不带连字符的 UUID 生成
- 随机密码生成
  - 支持自定义密码长度
  - 包含大小写字母和数字
- IP 查询

## 安装

```bash
cargo install weap
```

## 使用方法

### UUID 生成

```bash
# 生成带连字符的 UUID
weap uuid

# 生成不带连字符的 UUID
weap uuid -x
```

### 随机密码生成

```bash
# 生成默认长度(16位)的随机密码
weap pw

# 生成指定长度的随机密码
weap pw -l 20
```

### IP查询

```bash
# 默认使用本机IP查询
weap ip

# 指定ip查询
weap ip 8.8.8.8

# 输出示例
🌐 IP信息
├─ 地址: 58.152.116.89
├─ 位置: Central,Central and Western District(HCW),Hong Kong
├─ 国家: Hong Kong (HK)
├─ 时区: Asia/Hong_Kong
├─ 组织: Hong Kong Telecommunications (HKT) Limited
├─ ISP: Hong Kong Telecommunications (HKT) Limited Mass Internet
├─ ZIP: 96521
├─ AS: AS4760 HKT Limited
└─ 坐标: 22.2836,114.1600

```

## 许可证

MIT License 