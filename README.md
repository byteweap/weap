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
- **哈希计算**
  - 支持 MD5、SHA256、SHA512 算法
  - 支持文本和文件输入
- **Base64 编解码**
  - 支持 UTF-8 文本编解码
  - 支持中文等多字节字符
- **进制转换**
  - 支持 2-36 进制输入
  - 同时输出二进制、八进制、十进制、十六进制

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

### 哈希计算

```bash
# 计算文本的 SHA256（默认）
weap hash "hello"
# 输出
2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824

# 指定算法
weap hash -a md5 "hello"
weap hash -a sha512 "hello"

# 计算文件哈希
weap hash -f ./Cargo.toml
weap hash -a md5 -f ./Cargo.toml
```

### Base64 编解码

```bash
# 编码
weap b64 encode "hello world"
# 输出
aGVsbG8gd29ybGQ=

# 解码
weap b64 decode "aGVsbG8gd29ybGQ="
# 输出
hello world

# 支持中文
weap b64 encode "你好世界"
weap b64 decode "5L2g5aW95LiW55WM"
```

### 进制转换

```bash
# 十进制转各进制
weap base 255
# 输出
二进制 (bin):  11111111
八进制 (oct):  377
十进制 (dec):  255
十六进制 (hex): ff

# 从二进制输入
weap base -f 2 "11111111"

# 从十六进制输入
weap base -f 16 "ff"
```

## 许可证

MIT License