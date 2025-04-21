# weap

一个简单实用的命令行小工具集合，为开发者和系统管理员提供日常工作便利。

## 功能特性

- **UUID 生成**
  - 支持带连字符的 UUID 生成
  - 支持不带连字符的 UUID 生成
- **随机密码生成**
  - 支持自定义密码长度
  - 包含大小写字母和数字
- **IP 查询**
  - 支持本机ip查询
  - 支持指定ip查询
- **系统信息**
  - 显示操作系统信息
  - 显示CPU和内存使用情况
- **时间工具**
  - 显示当前时间和UTC时间
  - 支持查看不同时区时间
  - 提供倒计时功能

## 安装

### 通过 Cargo 安装

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

### 系统信息

```bash
# 显示系统信息
weap sys
# 输出示例
操作系统: Darwin
系统版本: 15.4.1
内核版本: 24.4.0
内存: 11.67/16.00 GB  使用率: 72.94%
CPU: 8c (Apple M1)

```

### 时间信息

```bash
# 1. 显示当前时间
weap time
# 输出
当前本地时间: 2025-04-21 14:25:10
当前UTC时间: 2025-04-21 06:25:10

# 2. 显示当前时间及秒级时间戳
weap time -u
# 输出
当前本地时间: 2025-04-21 14:26:38
当前UTC时间: 2025-04-21 06:26:38
Unix时间戳: 1745216798

# 3. 显示指定时区时间
weap time -t Asia/Shanghai
# 输出
当前本地时间: 2025-04-21 14:28:22
当前UTC时间: 2025-04-21 06:28:22
Asia/Shanghai时间: 2025-04-21 14:28:22

# 4. 自定义格式时间
weap time -f "%H:%M:%S"
# 输出
当前本地时间: 14:29:36
当前UTC时间: 06:29:36

# 5. 倒计时（以60s为例）
weap time -c 60
# 输出（剩余时间逐秒递减）
开始倒计时 60 秒
剩余时间: 00:55 

```

## 许可证

MIT License