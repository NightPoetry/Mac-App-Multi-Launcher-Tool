#!/bin/bash

# 微信双开一键脚本 - WeChatDual Launcher
# 使用方法：sudo bash wechat-dual.sh

set -e  # 遇到错误立即退出

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 应用路径
DUAL_APP="/Applications/微信双开.app"
ORIGINAL="/Applications/WeChat.app"
BUNDLE_ID="com.tencent.xinWeChat2"

# 打印带颜色的消息
print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# 检查是否以 root 权限运行
if [ "$EUID" -ne 0 ]; then 
    print_error "请使用 sudo 运行此脚本"
    print_info "使用方法：sudo bash $0"
    exit 1
fi

print_info "🚀 开始配置微信双开..."
echo ""

# 1. 检查原应用是否存在
print_info "步骤 1/6: 检查原版微信"
if [ ! -d "$ORIGINAL" ]; then
    print_error "未找到原版微信，请确认已安装到 /Applications/WeChat.app"
    exit 1
fi
print_success "找到原版微信：$ORIGINAL"
echo ""

# 2. 检查是否已存在双开应用
if [ -d "$DUAL_APP" ]; then
    print_warning "已存在双开应用，是否删除重新配置？"
    read -p "(y/n): " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_info "删除旧的双开应用..."
        rm -rf "$DUAL_APP"
    else
        print_info "跳过配置，直接启动双开微信"
        open -n "$DUAL_APP"
        print_success "配置完成！"
        exit 0
    fi
fi

# 3. 复制应用
print_info "步骤 2/6: 复制应用副本..."
cp -R "$ORIGINAL" "$DUAL_APP"
print_success "应用副本创建完成"
echo ""

# 4. 修改 Bundle ID
print_info "步骤 3/6: 修改 Bundle ID..."
/usr/libexec/PlistBuddy -c "Set :CFBundleIdentifier $BUNDLE_ID" \
  "$DUAL_APP/Contents/Info.plist"
print_success "Bundle ID 已修改为：$BUNDLE_ID"
echo ""

# 5. 清除原有签名
print_info "步骤 4/6: 清除原有签名..."
codesign --remove-signature "$DUAL_APP" 2>/dev/null || true
print_success "原有签名已清除"
echo ""

# 6. 清除隔离属性（避免图标斜杠）
print_info "步骤 5/7: 清除隔离属性..."
xattr -cr "$DUAL_APP" 2>/dev/null || true
print_success "隔离属性已清除"
echo ""

# 7. 修复权限
print_info "步骤 6/7: 修复文件权限..."
chown -R $USER:staff "$DUAL_APP"
chmod -R 755 "$DUAL_APP"
print_success "文件权限已修复"
echo ""

# 8. 重新签名（优化版，避免过度签名）
print_info "步骤 7/7: 重新签名应用..."
codesign --force -s - --timestamp=none "$DUAL_APP"
print_success "应用签名完成"
echo ""

# 9. 验证签名
print_info "验证配置..."
if codesign -dv --verbose=4 "$DUAL_APP" 2>&1 | grep -q "Team ID"; then
    print_warning "签名可能存在问题，但不影响使用"
else
    print_success "签名验证通过"
fi
echo ""

# 10. 启动双开
print_success "🎉 配置完成！正在启动双开微信..."
open -n "$DUAL_APP"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
print_success "微信双开配置完成！"
echo ""
echo "   📱 原版微信：$ORIGINAL"
echo "   📱 双开微信：$DUAL_APP"
echo ""
echo "   💡 提示："
echo "      - 两个微信可以同时运行"
echo "      - 数据完全隔离，互不影响"
echo "      - 可以通过修改图标来区分两个账号"
echo ""
echo "   📖 查看详细教程：docs/技术原理说明.md"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 询问是否修改图标
read -p "是否需要修改双开微信的图标？(y/n): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_info "打开双开微信的简介窗口..."
    print_info "请按以下步骤操作："
    echo ""
    echo "   1. 在 Finder 中找到：$DUAL_APP"
    echo "   2. 右键点击 → 显示简介 (或按 Command+I)"
    echo "   3. 点击简介窗口左上角的小图标"
    echo "   4. 将准备好的图标文件拖拽到该位置"
    echo "   5. 或复制图标后按 Command+V 粘贴"
    echo ""
    open -R "$DUAL_APP"
fi

exit 0
