#!/bin/bash

# 微信双开图标斜杠修复脚本
# 使用方法：bash fix-icon-slash.sh

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "   🔧 微信双开图标斜杠修复工具"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 查找双开微信
DUAL_APP=""

if [ -d "/Applications/微信双开.app" ]; then
    DUAL_APP="/Applications/微信双开.app"
elif [ -d "/Applications/WeChat2.app" ]; then
    DUAL_APP="/Applications/WeChat2.app"
elif [ -d "/Applications/WeChat_Dual.app" ]; then
    DUAL_APP="/Applications/WeChat_Dual.app"
else
    print_info "未找到常见的双开微信路径"
    print_info "请手动输入双开微信的路径（或按 Enter 退出）"
    read -p "路径：" DUAL_APP
    
    if [ -z "$DUAL_APP" ]; then
        print_error "未提供路径，退出脚本"
        exit 1
    fi
fi

# 验证路径
if [ ! -d "$DUAL_APP" ]; then
    print_error "找不到应用：$DUAL_APP"
    exit 1
fi

print_success "找到双开微信：$DUAL_APP"
echo ""

print_info "🔧 开始修复图标斜杠问题..."
echo ""

# 步骤 1: 清除隔离属性
print_info "步骤 1/3: 清除隔离属性（Quarantine）..."
if xattr -cr "$DUAL_APP" 2>/dev/null; then
    print_success "隔离属性已清除"
else
    print_warning "清除隔离属性失败（可能已清除）"
fi
echo ""

# 步骤 2: 修复文件权限
print_info "步骤 2/3: 修复文件权限..."
print_info "   - 修复文件所有者..."
if chown -R $USER:staff "$DUAL_APP" 2>/dev/null; then
    print_success "文件所有者已修复：$USER:staff"
else
    print_info "   需要管理员权限修复所有者..."
    if sudo chown -R $USER:staff "$DUAL_APP" 2>/dev/null; then
        print_success "文件所有者已修复（sudo）"
    else
        print_warning "无法修复文件所有者（跳过）"
    fi
fi

print_info "   - 修复文件权限..."
if chmod -R 755 "$DUAL_APP" 2>/dev/null; then
    print_success "文件权限已修复：755"
else
    print_info "   需要管理员权限修复权限..."
    if sudo chmod -R 755 "$DUAL_APP" 2>/dev/null; then
        print_success "文件权限已修复（sudo）"
    else
        print_warning "无法修复文件权限（跳过）"
    fi
fi
echo ""

# 步骤 3: 重新签名
print_info "步骤 3/3: 重新签名应用..."
if codesign --force -s - --timestamp=none "$DUAL_APP" 2>/dev/null; then
    print_success "应用签名完成"
else
    print_info "   需要管理员权限重新签名..."
    if sudo codesign --force -s - --timestamp=none "$DUAL_APP" 2>/dev/null; then
        print_success "应用签名完成（sudo）"
    else
        print_error "签名失败"
        exit 1
    fi
fi
echo ""

# 验证
print_info "验证修复结果..."
if codesign -dv --verbose=4 "$DUAL_APP" 2>&1 | grep -q "Team ID"; then
    print_warning "签名可能存在问题，但通常不影响使用"
else
    print_success "签名验证通过"
fi

# 检查隔离属性
if xattr -p com.apple.quarantine "$DUAL_APP" 2>/dev/null; then
    print_warning "隔离属性仍然存在，建议手动清除"
else
    print_success "隔离属性已完全清除"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
print_success "🎉 修复完成！"
echo ""
echo "💡 提示："
echo "   - 斜杠图标应该已经消失"
echo "   - 如仍有斜杠，请重启 Finder 或电脑"
echo "   - 斜杠不影响功能使用，可忽略"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 询问是否重启应用
read -p "是否现在重启双开微信？(y/n): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_info "重启双开微信..."
    killall -9 "WeChat" 2>/dev/null || true
    sleep 1
    open -n "$DUAL_APP"
    print_success "微信已启动"
fi

exit 0
