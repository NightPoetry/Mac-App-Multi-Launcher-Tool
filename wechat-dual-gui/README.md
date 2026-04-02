# 微信双开工具 GUI 版本

> 基于 Tauri (Rust + Web) 开发的可视化微信双开工具

---

## 🚀 快速开始

### 开发模式运行

```bash
cd wechat-dual-gui
npm install
npm run tauri dev
```

### 打包应用

```bash
npm run tauri build
```

打包后的应用位于：`src-tauri/target/release/bundle/macos/`

---

## ✨ 功能特性

### 1. 一键双开
- ✅ 自动复制微信应用
- ✅ 自动修改 Bundle ID
- ✅ 自动清除隔离属性（避免斜杠）
- ✅ 自动修复文件权限
- ✅ 自动重新签名
- ✅ 可选自定义图标

### 2. 应用管理
- 📋 查看所有双开应用
- 🚀 一键启动双开应用
- 🔧 一键修复图标斜杠
- 🗑️ 一键删除双开应用

### 3. 图标自定义
- 📁 支持选择自定义图标
- 🎨 支持 .icns、.png、.jpg 格式
- 💡 可视化图标选择

### 4. 实时日志
- 📝 显示操作进度
- ✅ 成功/失败提示
- 🎨 彩色日志输出

---

## 🎨 界面预览

### 主界面
- **状态检查**：显示微信安装状态
- **创建表单**：输入应用名称、Bundle ID、选择图标
- **应用列表**：显示所有双开应用
- **操作日志**：实时显示操作进度

### 操作按钮
- **创建双开微信**：一键创建
- **启动**：启动双开应用
- **修复**：修复图标斜杠
- **删除**：删除双开应用

---

## 🔧 技术架构

### 后端 (Rust)
- **Tauri Commands**：提供 API 接口
- **系统调用**：执行 shell 命令
- **文件操作**：复制、修改、删除文件

### 前端 (Vanilla JS)
- **HTML**：界面结构
- **CSS**：样式设计
- **JavaScript**：交互逻辑

### 通信机制
- **Tauri IPC**：前后端通信
- **异步调用**：非阻塞操作
- **错误处理**：完善的错误提示

---

## 📋 Rust 后端 API

### check_wechat_installed
检查微信是否已安装

```rust
fn check_wechat_installed() -> Result<bool, String>
```

### create_dual_wechat
创建双开微信

```rust
fn create_dual_wechat(
    app_name: String,
    bundle_id: String,
    custom_icon: Option<String>,
) -> Result<String, String>
```

### launch_dual_wechat
启动双开微信

```rust
fn launch_dual_wechat(app_name: String) -> Result<String, String>
```

### fix_icon_slash
修复图标斜杠

```rust
fn fix_icon_slash(app_name: String) -> Result<String, String>
```

### get_dual_wechat_list
获取双开应用列表

```rust
fn get_dual_wechat_list() -> Result<Vec<String>, String>
```

### delete_dual_wechat
删除双开应用

```rust
fn delete_dual_wechat(app_name: String) -> Result<String, String>
```

---

## 🎨 使用流程

### 1. 创建双开微信

1. 输入应用名称（如：微信双开）
2. 输入 Bundle ID（如：com.tencent.xinWeChat2）
3. 可选：点击"选择图标文件"按钮
4. 点击"创建双开微信"按钮
5. 等待创建完成

### 2. 管理双开应用

1. 在"已有双开应用"列表中查看
2. 点击"启动"按钮启动应用
3. 点击"修复"按钮修复图标斜杠
4. 点击"删除"按钮删除应用

### 3. 查看操作日志

1. 在"操作日志"区域查看实时日志
2. 日志显示操作进度和结果
3. 点击"清空日志"按钮清空日志

---

## 🔒 安全说明

### 权限要求
- 需要管理员权限执行系统命令
- 需要文件读写权限
-需要执行 shell 命令权限

### 安全机制
- 仅修改本地应用文件
- 不破解网络协议
- 不注入恶意代码
- 数据完全隔离

---

## 📦 开发说明

### 环境要求
- **Rust**：1.70+
- **Node.js**：18+
- **npm**：9+
- **macOS**：10.14+

### 开发命令

```bash
# 安装依赖
npm install

# 开发模式运行
npm run tauri dev

# 构建应用
npm run tauri build

# 检查代码
npm run tauri check
```

### 项目结构

```
wechat-dual-gui/
├── src/                    # 前端代码
│   ├── index.html          # HTML 结构
│   ├── main.js            # JavaScript 逻辑
│   └── styles.css         # CSS 样式
├── src-tauri/             # 后端代码
│   ├── src/
│   │   ├── main.rs        # Rust 入口
│   │   └── lib.rs        # Rust 核心逻辑
│   ├── Cargo.toml          # Rust 依赖配置
│   └── tauri.conf.json    # Tauri 配置
└── package.json            # Node.js 配置
```

---

## 🐛 常见问题

### Q: 提示"微信未安装"？

**A**: 请先从 App Store 或官网安装微信。

### Q: 创建失败？

**A**: 检查是否有管理员权限，查看日志了解详细错误。

### Q: 图标斜杠仍然存在？

**A**: 点击"修复"按钮，或手动运行修复脚本。

### Q: 应用无法启动？

**A**: 检查 Bundle ID 是否正确，尝试重新创建。

---

## 📞 反馈与支持

### 问题反馈
- 提交 GitHub Issue
- 查看操作日志
- 检查系统权限

### 技术支持
- 查看技术原理说明文档
- 查看快速开始文档
- 查看更新日志

---

## 📄 开源协议

本项目采用 **GPL-3.0** 协议开源。

详细协议内容请查看项目根目录的 LICENSE 文件。

---

## 🌟 项目特色

- ✅ **可视化操作**：无需命令行
- ✅ **一键双开**：自动化完成所有步骤
- ✅ **自动修复**：自动清除斜杠
- ✅ **图标自定义**：可视化选择图标
- ✅ **实时日志**：查看操作进度
- ✅ **应用管理**：启动、修复、删除
- ✅ **跨平台**：基于 Tauri，支持多平台

---

**项目状态**: ✅ 开发完成  
**版本**: 1.0.0  
**最后更新**: 2026-03-31  
**维护状态**: 🟢 活跃维护中

---

*感谢使用微信双开工具 GUI 版本！*
