#!/bin/bash

# 测试 ICNS 转换流程

echo "=== 测试 ICNS 转换流程 ==="

# 创建临时目录
TEMP_DIR="/tmp/iconset-test-$$"
ICONSET_DIR="$TEMP_DIR/icon.iconset"

echo "1. 创建临时目录：$ICONSET_DIR"
mkdir -p "$ICONSET_DIR"

# 源图片
SOURCE_IMAGE="/Volumes/Project/WeChatDouble/wechat-dual-gui/src-tauri/icons/icon.png"

if [ ! -f "$SOURCE_IMAGE" ]; then
    echo "错误：源图片不存在：$SOURCE_IMAGE"
    exit 1
fi

echo "2. 源图片：$SOURCE_IMAGE"

# 生成各种尺寸的图标
echo "3. 生成各种尺寸的图标..."

sizes=(
    "16:icon_16x16.png"
    "32:icon_16x16@2x.png"
    "32:icon_32x32.png"
    "64:icon_32x32@2x.png"
    "128:icon_128x128.png"
    "256:icon_128x128@2x.png"
    "256:icon_256x256.png"
    "512:icon_256x256@2x.png"
    "512:icon_512x512.png"
    "1024:icon_512x512@2x.png"
)

for item in "${sizes[@]}"; do
    size="${item%%:*}"
    filename="${item##*:}"
    output_path="$ICONSET_DIR/$filename"
    
    echo "  - 生成 ${size}x${size}: $filename"
    sips -z "$size" "$size" "$SOURCE_IMAGE" --out "$output_path" > /dev/null 2>&1
    
    if [ ! -f "$output_path" ]; then
        echo "    错误：文件未创建"
        exit 1
    fi
done

echo "4. 检查 iconset 目录内容:"
ls -1 "$ICONSET_DIR"

echo "5. 使用 iconutil 生成 ICNS..."
OUTPUT_ICNS="$TEMP_DIR/output.icns"
iconutil -c icns "$ICONSET_DIR" -o "$OUTPUT_ICNS"

if [ ! -f "$OUTPUT_ICNS" ]; then
    echo "错误：ICNS 文件未生成"
    exit 1
fi

echo "6. ICNS 文件生成成功!"
ls -lh "$OUTPUT_ICNS"

echo "7. 清理临时文件..."
rm -rf "$TEMP_DIR"

echo "=== 测试完成 ==="
