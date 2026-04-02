# GitHub项目推送完整流程指南

本指南总结了将本地项目推送到GitHub仓库的完整流程，适用于指导AI助手执行。

## 前提条件

- 已有GitHub仓库地址
- 本地项目目录
- Git已安装

## 完整流程步骤

### 1. 检查Git仓库状态

```bash
git status
```

**目的**：检查当前目录是否已初始化为git仓库

**可能结果**：
- `fatal: not a git directory` - 需要初始化
- `On branch main` - 已初始化，继续下一步

### 2. 初始化Git仓库（如需要）

```bash
git init
```

**目的**：将当前目录初始化为git仓库

### 3. 设置仓库描述（可选）

```bash
git config core.description '你的项目描述'
```

**目的**：设置仓库描述，会在GitHub上显示

**示例**：
```bash
git config core.description 'macOS应用多开技术研究 / macOS App Multi-instance Research
纯技术研究项目，探索应用多开、签名修改、图标定制技术原理，以微信为示范样例
Research project exploring app multi-instance, code signing, icon customization on macOS'
```

### 4. 添加所有文件到Git

```bash
git add .
```

**目的**：将所有文件添加到暂存区

### 5. 创建初始提交

```bash
git commit -m "Initial commit: 项目描述"
```

**目的**：创建第一个提交

**示例**：
```bash
git commit -m "Initial commit: macOS应用多开技术研究项目"
```

### 6. 添加远程仓库地址

```bash
git remote add origin https://github.com/用户名/仓库名.git
```

**目的**：关联远程GitHub仓库

**示例**：
```bash
git remote add origin https://github.com/NightPoetry/Mac-App-Multi-Launcher-Tool.git
```

### 7. 推送到GitHub（首次推送）

```bash
git push -u origin main
```

**目的**：将本地提交推送到GitHub

**可能遇到的问题**：

#### 问题1：远程仓库已有内容
```
! [rejected]        main -> main (fetch first)
```

**解决方案**：
```bash
git pull origin main --allow-unrelated-histories --no-rebase
```

#### 问题2：合并冲突
```
CONFLICT (add/add): Merge conflict in LICENSE
```

**解决方案**：
```bash
# 查看冲突文件
git status

# 选择保留本地版本
git checkout --ours LICENSE

# 标记冲突已解决
git add LICENSE

# 完成合并
git commit -m "Merge remote repository with local changes, keeping local version"
```

#### 问题3：需要指定合并策略
```
fatal: Need to specify how to reconcile divergent branches.
```

**解决方案**：
```bash
git pull origin main --allow-unrelated-histories --no-rebase
```

### 8. 处理冲突后再次推送

```bash
git push -u origin main
```

### 9. 验证推送结果

```bash
git status
```

**预期结果**：
```
On branch main
Your branch is up to date with 'origin/main'.
nothing to commit, working tree clean
```

## 常用Git命令速查

### 查看当前状态
```bash
git status
```

### 查看提交历史
```bash
git log --oneline
```

### 查看远程仓库
```bash
git remote -v
```

### 查看仓库描述
```bash
git config core.description
```

### 添加特定文件
```bash
git add 文件名
```

### 提交更改
```bash
git commit -m "提交信息"
```

### 推送更改
```bash
git push origin main
```

### 拉取远程更改
```bash
git pull origin main
```

## 完整示例脚本

```bash
#!/bin/bash

# 设置变量
REPO_URL="https://github.com/用户名/仓库名.git"
DESCRIPTION="项目描述"
COMMIT_MSG="Initial commit: 项目描述"

# 1. 初始化仓库
git init

# 2. 设置描述
git config core.description "$DESCRIPTION"

# 3. 添加所有文件
git add .

# 4. 创建提交
git commit -m "$COMMIT_MSG"

# 5. 添加远程仓库
git remote add origin "$REPO_URL"

# 6. 尝试推送
if ! git push -u origin main; then
    # 如果推送失败，尝试拉取并合并
    git pull origin main --allow-unrelated-histories --no-rebase
    
    # 如果有冲突，选择本地版本
    if [ -f LICENSE ]; then
        git checkout --ours LICENSE
        git add LICENSE
        git commit -m "Merge remote repository with local changes"
    fi
    
    # 再次推送
    git push -u origin main
fi

# 7. 验证结果
git status
```

## 注意事项

1. **分支名称**：现代Git默认使用`main`分支，旧版本可能使用`master`
2. **认证问题**：如果遇到认证错误，需要配置GitHub Personal Access Token
3. **冲突处理**：根据实际情况选择保留本地或远程版本
4. **大文件**：如果项目包含大文件，可能需要使用Git LFS
5. **.gitignore**：确保有合适的.gitignore文件，避免提交不必要的文件

## 给AI助手的指令模板

```
请帮我将当前项目推送到GitHub仓库：https://github.com/用户名/仓库名.git

要求：
1. 检查并初始化git仓库（如需要）
2. 设置仓库描述为：[描述内容]
3. 添加所有文件并创建初始提交
4. 关联远程仓库
5. 处理可能出现的冲突和错误
6. 成功推送到GitHub
7. 验证推送结果

请按照标准git流程执行，遇到问题时自动处理。
```

## 故障排除

### 问题：fatal: not a git directory
**解决**：运行 `git init`

### 问题：Permission denied
**解决**：检查GitHub仓库权限，配置认证信息

### 问题：SSL certificate problem
**解决**：
```bash
git config --global http.sslVerify false
```

### 问题：Connection refused
**解决**：检查网络连接和仓库地址是否正确

---

**最后更新**：2026-04-02
**适用版本**：Git 2.x+
**适用平台**：macOS, Linux, Windows (Git Bash)
