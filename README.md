# 格式匠

一款面向学术与专业写作人员的 Markdown 转 Word 工具，结合 Tauri 桌面框架与阿里云 EMAS Serverless 云服务，提供零配置、专业级的文档转换体验。


## 📋 功能特性

### 核心功能
- **Markdown 转 Word**：支持 `.md`、`.zip` 等多种格式，支持文本框直接输入，一键转换为 `.docx`。
- **丰富模板**：内置多款精美模板，支持模板搜索与预览 (TODO)
- **AI 格式修复**（付费）：自动修复 Markdown 格式问题，提升输出质量
- **自定义模板**（付费）：上传文档自动识别样式，生成个性化模板

### 平台特性
- **零依赖安装**：内置 Pandoc 环境，无需额外配置
- **跨平台支持**：Windows、macOS、Linux 完整支持
- **离线转换**：核心转换功能无需网络连接

## 🚀 快速开始

### 前置要求
- Node.js 16+
- Rust 1.70+
- pnpm 或 npm

### 安装与运行

```bash
# 安装依赖
pnpm install

# 开发模式运行
pnpm tauri dev

# 构建发布版本
pnpm tauri build
```

## 🔗 相关链接

(TODO)
- 用户登录：https://linjhs.com/login
- 用户注册：https://linjhs.com/signup
- 商品列表：https://linjhs.com/shop/products

## 📝 许可证

- **开源部分**：GPL-3.0 License
- **付费部分**：专有许可

## 💳 会员体系

| 功能特性     | 基础版（免费） | 专业版（¥2.9/月） | 大师版（¥26.9/月） |
| ------------ | -------------- | ----------------- | ------------------ |
| 基础格式转换 | ✅              | ✅                 | ✅                  |
| 默认模板     | 1-2 款         | 全部高级模板      | 全部高级模板       |
| AI 格式修复  | ❌              | ❌                 | ✅                  |
| 自定义模板   | ❌              | ✅                 | ✅                  |
| 优先技术支持 | ❌              | ❌                 | ✅                  |


## 🤝 贡献指南

欢迎提交 Issue 与 Pull Request！本项目使用 GitHub 工作流构建。

## 📧 反馈与支持

- 提交 Issue：[GitHub Issues](https://github.com/LinJHS/format-tools/issues)
- 官方邮箱：2360342887@qq.com
- 社区讨论：欢迎加入用户交流群 (TODO)

**致力于让专业写作更简单。** ✨
