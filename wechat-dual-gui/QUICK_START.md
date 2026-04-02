# 微信分身工具 - 快速开始指南

> 📖 完整的开发、调试、打包指南

---

## 📋 目录

1. [环境要求](#环境要求)
2. [安装依赖](#安装依赖)
3. [开发模式](#开发模式运行)
4. [调试技巧](#调试技巧)
5. [打包应用](#打包应用)
6. [故障排查](#故障排查)

---

## 🛠 环境要求

### 必需软件

| 软件 | 版本要求 | 用途 |
|------|---------|------|
| **Node.js** | 18.0+ | 前端开发环境 |
| **npm** | 9.0+ | 包管理器 |
| **Rust** | 1.70+ | 后端开发语言 |
| **macOS** | 10.14+ | 操作系统 |
| **微信** | 最新版 | 需要双开的目标应用 |

### 检查环境

```bash
# 检查 Node.js 版本
node -v

# 检查 npm 版本
npm -v

# 检查 Rust 版本
rustc --version

# 检查 Cargo 版本
cargo --version
```

### 安装缺失的环境

#### 1. 安装 Node.js

**方法一：使用官网安装包**
```bash
# 访问 https://nodejs.org/ 下载 macOS 安装包
```

**方法二：使用 Homebrew**
```bash
brew install node
```

#### 2. 安装 Rust

```bash
# 使用官方安装脚本
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装完成后重启终端或运行
source $HOME/.cargo/env
```

#### 3. 安装 Xcode 命令行工具

```bash
xcode-select --install
```

---

## 📦 安装依赖

### 1. 进入项目目录

```bash
cd /Volumes/Project/WeChatDouble/wechat-dual-gui
```

### 2. 安装 Node.js 依赖

```bash
npm install
```

**安装的内容：**
- `@tauri-apps/cli` - Tauri 开发工具
- `cropperjs` - 图片裁剪库

### 3. 安装 Rust 依赖

```bash
# 首次运行 Tauri 命令时会自动安装
cargo install --path src-tauri
```

---

## 🚀 开发模式运行

### 启动开发服务器

```bash
npm run tauri dev
```

**这个过程会：**
1. 编译 Rust 后端代码
2. 启动前端开发服务器
3. 自动打开应用窗口
4. 监听文件变化并热重载

### 首次启动时间

- **Rust 编译**：约 2-5 分钟（取决于电脑性能）
- **后续编译**：约 10-30 秒（增量编译）

### 开发模式特色

- ✅ 文件修改后自动重载
- ✅ 前端代码实时刷新
- ✅ 后端代码修改后重新编译
- ✅ 完整的日志输出

---

## 🐛 调试技巧

### 1. 前端调试

**打开开发者工具：**
- 在应用窗口右键 → "检查"
- 或使用快捷键 `Cmd + Option + I`

**在开发者工具中：**
- 查看 Console 日志
- 调试 JavaScript 代码
- 检查 HTML/CSS
- 查看网络请求

### 2. 后端调试

**查看 Rust 日志：**
```bash
# 在开发模式下，日志会输出到终端
# 使用 RUST_LOG 环境变量控制日志级别
RUST_LOG=debug npm run tauri dev
```

**日志级别：**
- `error` - 错误信息
- `warn` - 警告信息
- `info` - 一般信息
- `debug` - 调试信息
- `trace` - 详细追踪

### 3. 查看操作日志

应用内置了操作日志面板：
- 实时显示所有操作
- 显示成功/失败状态
- 包含详细的错误信息

### 4. 调试常见问题

#### 问题：Rust 代码修改后没有生效

**解决方案：**
```bash
# 停止开发服务器 (Ctrl+C)
# 清理编译缓存
cd src-tauri
cargo clean
cd ..
# 重新启动
npm run tauri dev
```

#### 问题：前端样式没有更新

**解决方案：**
- 硬刷新：`Cmd + Shift + R`
- 或清除浏览器缓存

#### 问题：端口被占用

**解决方案：**
```bash
# 查看占用端口的进程
lsof -i :1420

# 杀死进程
kill -9 <PID>
```

---

## 📦 打包应用

### 1. 构建 Release 版本

```bash
npm run tauri build
```

**构建过程：**
1. 编译优化后的 Rust 代码
2. 打包前端资源
3. 生成应用安装包
4. 代码签名（如果配置了）

### 2. 构建产物位置

构建完成后，文件位于：

```
src-tauri/target/release/bundle/macos/
├── WeChatFenshen.app          # 可直接运行的应用
├── WeChatFenshen_1.0.0_x64.dmg    # DMG 安装包
└── .app.tar.gz                # 压缩版本
```

### 3. 测试打包的应用

```bash
# 直接运行 .app 文件
open src-tauri/target/release/bundle/macos/WeChatFenshen.app
```

### 4. 构建选项

#### 仅构建不打包

```bash
cd src-tauri
cargo build --release
```

#### 仅打包

```bash
cd src-tauri
cargo tauri build --bundles dmg
```

#### 指定架构

```bash
# Apple Silicon (M1/M2)
cargo tauri build --target aarch64-apple-darwin

# Intel Mac
cargo tauri build --target x86_64-apple-darwin
```

### 5. 代码签名（可选）

如果需要分发应用，需要进行代码签名：

#### 获取开发者证书

1. 打开 Xcode
2. 偏好设置 → 账户
3. 添加 Apple ID
4. 管理证书 → 创建 macOS 应用签名证书

#### 配置签名

编辑 `src-tauri/Info.plist`：

```xml
<key>CFBundleIdentifier</key>
<string>com.yourcompany.wechatfenshen</string>
```

#### 手动签名

```bash
# 签名应用
codesign --force --deep -s "Your Developer ID" \
  src-tauri/target/release/bundle/macos/WeChatFenshen.app
```

---

## 🔧 故障排查

### 常见问题及解决方案

#### 1. `npm run tauri dev` 失败

**错误：`command not found: tauri`**

```bash
# 重新安装依赖
npm install
# 或全局安装 Tauri CLI
npm install -g @tauri-apps/cli
```

#### 2. Rust 编译错误

**错误：`package 'tauri' required but not found`**

```bash
# 更新 Rust 工具链
rustup update
# 清理并重新编译
cargo clean
cargo build
```

#### 3. 权限错误

**错误：`Permission denied`**

```bash
# 修复项目权限
sudo chown -R $USER:staff /Volumes/Project/WeChatDouble/wechat-dual-gui
```

#### 4. 微信路径错误

**错误：`/Applications/WeChat.app not found`**

**解决方案：**
- 确认微信已安装在 `/Applications` 目录
- 如果微信在其他位置，需要修改代码中的路径

#### 5. 图标不显示

**解决方案：**
```bash
# 清除图标缓存
touch /Applications/微信分身.app
killall Finder
```

#### 6. 应用启动失败

**错误：`The application can't be opened`**

```bash
# 移除隔离属性
xattr -cr /Applications/微信分身.app

# 重新签名
codesign --force --deep -s - /Applications/微信分身.app
```

---

## 📝 开发工作流建议

### 日常开发

1. **启动开发服务器**
   ```bash
   npm run tauri dev
   ```

2. **修改代码**
   - 前端：修改 `src/` 下的文件
   - 后端：修改 `src-tauri/src/` 下的文件

3. **查看效果**
   - 前端修改立即生效
   - 后端修改需要等待重新编译

4. **测试功能**
   - 在应用中测试新功能
   - 查看操作日志确认结果

### 发布前准备

1. **代码检查**
   ```bash
   # Rust 代码检查
   cd src-tauri
   cargo clippy
   
   # 格式化代码
   cargo fmt
   ```

2. **功能测试**
   - 测试所有功能
   - 确认没有错误日志

3. **构建 Release**
   ```bash
   npm run tauri build
   ```

4. **测试打包的应用**
   - 安装 DMG 文件
   - 测试完整功能

---

## 🎯 性能优化

### 加快编译速度

1. **使用 M1/M2 Mac**
   - Apple Silicon 编译速度更快

2. **启用编译缓存**
   ```bash
   # Cargo 会自动缓存，无需特殊配置
   ```

3. **只编译修改的部分**
   - Tauri 会自动检测变化
   - 避免不必要的修改

### 减小应用体积

1. **移除未使用的依赖**
   ```bash
   cargo audit
   ```

2. **优化图片资源**
   - 压缩图标文件
   - 使用合适的分辨率

---

## 📚 相关资源

### 官方文档

- [Tauri 官方文档](https://tauri.app/)
- [Rust 官方文档](https://www.rust-lang.org/)
- [Node.js 官方文档](https://nodejs.org/)

### 社区资源

- [Tauri GitHub](https://github.com/tauri-apps/tauri)
- [Rust 中文社区](https://rustcc.cn/)

---

## 🆘 获取帮助

### 遇到问题？

1. **查看日志**
   - 应用内的操作日志
   - 终端的输出信息

2. **搜索 Issue**
   - GitHub Issues
   - Stack Overflow

3. **提交问题**
   - 提供详细的错误信息
   - 附上操作日志
   - 说明系统环境

---

**最后更新**: 2026-04-02  
**版本**: 1.0.0  
**维护状态**: 🟢 活跃维护中

---

*祝你开发顺利！* 🎉
