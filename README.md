# weap

一个简单实用的命令行小工具

## 功能特性

- UUID 生成
  - 支持带连字符的 UUID 生成
  - 支持不带连字符的 UUID 生成
- 随机密码生成
  - 支持自定义密码长度
  - 包含大小写字母和数字

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

## 许可证

MIT License 