use std::fs;
use std::path::Path;
use std::process::Command;
use std::collections::HashSet;

const MARKER_FILE: &str = ".wechat-dual-marker";
const MARKER_CONTENT: &str = "This is a WeChat dual-instance created by WeChatDualTool";

fn convert_png_to_icns(png_path: &str, icns_path: &str) -> Result<(), String> {
    let temp_dir = std::env::temp_dir().join(format!("iconset-{}", std::process::id()));
    let iconset_path = temp_dir.join("icon.iconset");
    
    fs::create_dir_all(&iconset_path)
        .map_err(|e| format!("创建临时目录失败：{}", e))?;
    
    let sizes = vec![
        (16, "icon_16x16.png"),
        (32, "icon_16x16@2x.png"),
        (32, "icon_32x32.png"),
        (64, "icon_32x32@2x.png"),
        (128, "icon_128x128.png"),
        (256, "icon_128x128@2x.png"),
        (256, "icon_256x256.png"),
        (512, "icon_256x256@2x.png"),
        (512, "icon_512x512.png"),
        (1024, "icon_512x512@2x.png"),
    ];
    
    for (size, filename) in sizes {
        let output_path = iconset_path.join(filename);
        let output = Command::new("sips")
            .arg("-z")
            .arg(size.to_string())
            .arg(size.to_string())
            .arg(png_path)
            .arg("--out")
            .arg(&output_path)
            .output()
            .map_err(|e| format!("缩放图标失败：{}", e))?;
        
        if !output.status.success() {
            let _ = fs::remove_dir_all(&temp_dir);
            return Err(format!("缩放图标失败：{}", String::from_utf8_lossy(&output.stderr)));
        }
        
        if !output_path.exists() {
            let _ = fs::remove_dir_all(&temp_dir);
            return Err(format!("缩放后的文件未创建：{}", filename));
        }
    }
    
    let output = Command::new("iconutil")
        .arg("-c")
        .arg("icns")
        .arg(&iconset_path)
        .arg("-o")
        .arg(icns_path)
        .output()
        .map_err(|e| format!("生成 icns 失败：{}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let _ = fs::remove_dir_all(&temp_dir);
        return Err(format!("生成 icns 失败：{}{}", stdout, stderr));
    }
    
    let _ = fs::remove_dir_all(&temp_dir);
    
    if !Path::new(icns_path).exists() {
        return Err("ICNS 文件未生成".to_string());
    }
    
    Ok(())
}

#[tauri::command]
fn read_file_binary(path: String) -> Result<Vec<u8>, String> {
    fs::read(&path).map_err(|e| format!("读取文件失败：{}", e))
}

#[tauri::command]
fn write_file_binary(path: String, data: Vec<u8>) -> Result<(), String> {
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent).map_err(|e| format!("创建目录失败：{}", e))?;
    }
    fs::write(&path, &data).map_err(|e| format!("写入文件失败：{}", e))
}

#[tauri::command]
fn check_wechat_installed() -> Result<bool, String> {
    let wechat_path = Path::new("/Applications/WeChat.app");
    Ok(wechat_path.exists())
}

fn get_app_container_path(app_path: &Path) -> Option<String> {
    let app_name = app_path.file_stem()?.to_str()?;
    let home = std::env::var("HOME").ok()?;
    Some(format!("{}/Library/Containers/{}", home, app_name))
}

fn write_marker_file(app_path: &Path) -> Result<(), String> {
    if let Some(container_path) = get_app_container_path(app_path) {
        let marker_path = Path::new(&container_path).join(MARKER_FILE);
        fs::create_dir_all(&container_path)
            .map_err(|e| format!("创建容器目录失败：{}", e))?;
        fs::write(&marker_path, MARKER_CONTENT)
            .map_err(|e| format!("写入标记文件失败：{}", e))?;
        Ok(())
    } else {
        Err("无法获取容器路径".to_string())
    }
}

fn has_marker_file(app_name: &str) -> bool {
    if let Some(container_path) = get_app_container_path(Path::new(&format!("{}.app", app_name))) {
        let marker_path = Path::new(&container_path).join(MARKER_FILE);
        marker_path.exists()
    } else {
        false
    }
}

fn is_wechat_dual_app(app_path: &Path) -> bool {
    if app_path.extension().and_then(|s| s.to_str()) != Some("app") {
        return false;
    }

    let app_name = match app_path.file_stem().and_then(|s| s.to_str()) {
        Some(name) => name,
        None => return false,
    };

    if app_name == "WeChat" {
        return false;
    }

    if app_name.contains("WeChat") || app_name.contains("微信") {
        return true;
    }

    if has_marker_file(app_name) {
        return true;
    }

    let plist_path = app_path.join("Contents/Info.plist");
    if plist_path.exists() {
        if let Ok(output) = Command::new("/usr/libexec/PlistBuddy")
            .arg("-c")
            .arg("Print :CFBundleIdentifier")
            .arg(&plist_path)
            .output()
        {
            if output.status.success() {
                let bundle_id = String::from_utf8_lossy(&output.stdout);
                if bundle_id.contains("tencent") && bundle_id.contains("WeChat") {
                    return true;
                }
            }
        }
    }

    false
}

fn generate_unique_bundle_id(base_bundle_id: &str) -> Result<String, String> {
    let mut bundle_id = base_bundle_id.to_string();
    let mut counter = 2;
    
    while bundle_id_exists(&bundle_id) {
        bundle_id = format!("{}.v{}", base_bundle_id, counter);
        counter += 1;
    }
    
    Ok(bundle_id)
}

fn bundle_id_exists(bundle_id: &str) -> bool {
    let apps_dir = Path::new("/Applications");
    if let Ok(entries) = fs::read_dir(apps_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("app") {
                let plist_path = path.join("Contents/Info.plist");
                if plist_path.exists() {
                    if let Ok(output) = Command::new("/usr/libexec/PlistBuddy")
                        .arg("-c")
                        .arg("Print :CFBundleIdentifier")
                        .arg(&plist_path)
                        .output()
                    {
                        if output.status.success() {
                            let existing_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
                            if existing_id == bundle_id {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

#[tauri::command]
fn create_dual_wechat(
    app_name: String,
    bundle_id: String,
    custom_icon: Option<String>,
) -> Result<String, String> {
    let original_path = Path::new("/Applications/WeChat.app");
    let dual_path_str = format!("/Applications/{}.app", app_name);
    let dual_path = Path::new(&dual_path_str);

    if !original_path.exists() {
        return Err("未找到原版微信，请确认已安装".to_string());
    }

    if dual_path.exists() {
        return Err("分身应用已存在，请先删除".to_string());
    }

    let unique_bundle_id = generate_unique_bundle_id(&bundle_id)?;
    
    let mut steps = Vec::new();

    steps.push("开始创建分身微信...".to_string());

    steps.push(format!("步骤 1/7: 复制应用副本到 {}", app_name));
    let output = Command::new("cp")
        .arg("-R")
        .arg(original_path)
        .arg(&dual_path)
        .output()
        .map_err(|e| format!("复制应用失败：{}", e))?;

    if !output.status.success() {
        return Err(format!("复制应用失败：{:?}", String::from_utf8_lossy(&output.stderr)));
    }

    steps.push("应用副本创建完成".to_string());

    steps.push(format!("步骤 2/7: 修改 Bundle ID 为 {}", unique_bundle_id));
    let plist_path = dual_path.join("Contents/Info.plist");
    let output = Command::new("/usr/libexec/PlistBuddy")
        .arg("-c")
        .arg(format!("Set :CFBundleIdentifier {}", unique_bundle_id))
        .arg(&plist_path)
        .output()
        .map_err(|e| format!("修改 Bundle ID 失败：{}", e))?;

    if !output.status.success() {
        return Err(format!("修改 Bundle ID 失败：{:?}", String::from_utf8_lossy(&output.stderr)));
    }

    steps.push(format!("Bundle ID 已修改为：{}", unique_bundle_id));

    steps.push("步骤 3/7: 清除原有签名".to_string());
    let _ = Command::new("codesign")
        .arg("--remove-signature")
        .arg(&dual_path)
        .output();

    steps.push("原有签名已清除".to_string());

    steps.push("步骤 4/7: 清除隔离属性".to_string());
    let _ = Command::new("xattr")
        .arg("-cr")
        .arg(&dual_path)
        .output();

    steps.push("隔离属性已清除".to_string());

    steps.push("步骤 5/7: 修复文件权限".to_string());
    let username = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    
    let _ = Command::new("chown")
        .arg("-R")
        .arg(format!("{}:staff", username))
        .arg(&dual_path)
        .output();

    let _ = Command::new("chmod")
        .arg("-R")
        .arg("755")
        .arg(&dual_path)
        .output();

    steps.push("文件权限已修复".to_string());

    steps.push("步骤 6/7: 重新签名应用".to_string());
    let output = Command::new("codesign")
        .arg("--force")
        .arg("-s")
        .arg("-")
        .arg("--timestamp=none")
        .arg(&dual_path)
        .output()
        .map_err(|e| {
            format!("重新签名失败：{}。\n\n请尝试以下方法：\n1. 在 Finder 中找到 /Applications/{}.app\n2. 右键点击 -> 显示简介\n3. 解锁（点击左下角锁图标）\n4. 确保你的用户有读写权限\n5. 或者在终端运行：sudo chown -R $USER:staff /Applications/{}.app", 
                e, app_name, app_name)
        })?;

    if !output.status.success() {
        return Err(format!("重新签名失败：{:?}", String::from_utf8_lossy(&output.stderr)));
    }

    steps.push("应用签名完成".to_string());

    steps.push("步骤 7/7: 写入标记文件".to_string());
    write_marker_file(&dual_path)?;
    steps.push("标记文件已写入".to_string());

    if let Some(icon_path) = custom_icon {
        steps.push(format!("步骤 8/8: 转换并替换应用图标：{}", icon_path));
        let icon_source = Path::new(&icon_path);
        let icon_dest = dual_path.join("Contents/Resources/AppIcon.icns");

        if icon_source.exists() {
            convert_png_to_icns(&icon_path, &icon_dest.to_string_lossy())?;
            steps.push("图标格式已转换为.icns".to_string());
            
            let plist_path = dual_path.join("Contents/Info.plist");
            let _ = Command::new("/usr/libexec/PlistBuddy")
                .arg("-c")
                .arg("Set :CFBundleIconFile AppIcon.icns")
                .arg(&plist_path)
                .output();
            steps.push("图标配置已更新".to_string());
            
            let _ = Command::new("touch")
                .arg(&dual_path)
                .output();
            steps.push("图标缓存已刷新".to_string());
        } else {
            steps.push("图标文件不存在，跳过".to_string());
        }
    }

    steps.push("🎉 配置完成！".to_string());

    Ok(steps.join("\n"))
}

#[tauri::command]
fn update_dual_wechat(
    app_name: String,
    new_app_name: Option<String>,
    new_icon: Option<String>,
) -> Result<String, String> {
    let app_path_str = format!("/Applications/{}.app", app_name);
    let app_path = Path::new(&app_path_str);

    if !app_path.exists() {
        return Err("应用不存在".to_string());
    }

    let mut steps = Vec::new();
    steps.push(format!("开始更新应用：{}", app_name));

    if let Some(new_name) = new_app_name {
        if new_name != app_name {
            steps.push(format!("步骤 1/3: 重命名应用为 {}", new_name));
            
            let new_app_path_str = format!("/Applications/{}.app", new_name);
            let new_app_path = Path::new(&new_app_path_str);
            
            if new_app_path.exists() {
                return Err(format!("应用 {} 已存在", new_name));
            }

            fs::rename(app_path, new_app_path)
                .map_err(|e| format!("重命名应用失败：{}", e))?;
            
            steps.push("应用重命名完成".to_string());
            
            if let Some(icon_path) = new_icon {
                steps.push(format!("步骤 2/3: 转换并替换应用图标：{}", icon_path));
                let icon_source = Path::new(&icon_path);
                let icon_dest = new_app_path.join("Contents/Resources/AppIcon.icns");

                if icon_source.exists() {
                    convert_png_to_icns(&icon_path, &icon_dest.to_string_lossy())?;
                    steps.push("图标格式已转换为.icns".to_string());
                } else {
                    steps.push("图标文件不存在，跳过".to_string());
                }
                
                steps.push("步骤 3/3: 重新签名应用".to_string());
            } else {
                steps.push("步骤 2/2: 重新签名应用".to_string());
            }

            let username = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
            
            let _ = Command::new("chown")
                .arg("-R")
                .arg(format!("{}:staff", username))
                .arg(&new_app_path)
                .output();

            let _ = Command::new("chmod")
                .arg("-R")
                .arg("755")
                .arg(&new_app_path)
                .output();

            let output = Command::new("osascript")
                .arg("-e")
                .arg(format!(r#"do shell script "chown -R {}:staff '{}' && chmod -R 755 '{}'" with administrator privileges"#, 
                    username, new_app_path.display(), new_app_path.display()))
                .output()
                .map_err(|e| format!("请求管理员权限失败：{}", e))?;

            if !output.status.success() {
                return Err("用户取消了权限请求".to_string());
            }

            let output = Command::new("codesign")
                .arg("--force")
                .arg("-s")
                .arg("-")
                .arg("--timestamp=none")
                .arg(&new_app_path)
                .output()
                .map_err(|e| {
                    format!("重新签名失败：{}。\n\n请尝试以下方法：\n1. 在 Finder 中找到 /Applications/{}.app\n2. 右键点击 -> 显示简介\n3. 解锁（点击左下角锁图标）\n4. 确保你的用户有读写权限\n5. 或者在终端运行：sudo chown -R $USER:staff /Applications/{}.app", 
                        e, app_name, app_name)
                })?;

            if !output.status.success() {
                return Err(format!("重新签名失败：{:?}", String::from_utf8_lossy(&output.stderr)));
            }

            steps.push("应用签名完成".to_string());
        } else {
            if let Some(icon_path) = new_icon {
                steps.push(format!("步骤 1/2: 转换并替换应用图标：{}", icon_path));
                let icon_source = Path::new(&icon_path);
                let icon_dest = app_path.join("Contents/Resources/AppIcon.icns");

                if icon_source.exists() {
                    convert_png_to_icns(&icon_path, &icon_dest.to_string_lossy())?;
                    steps.push("图标格式已转换为.icns".to_string());
                } else {
                    steps.push("图标文件不存在，跳过".to_string());
                }
                
                steps.push("步骤 2/2: 重新签名应用".to_string());
            } else {
                return Err("至少需要修改名称或图标".to_string());
            }

            let username = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
            
            let _ = Command::new("chown")
                .arg("-R")
                .arg(format!("{}:staff", username))
                .arg(&app_path)
                .output();

            let _ = Command::new("chmod")
                .arg("-R")
                .arg("755")
                .arg(&app_path)
                .output();

            let output = Command::new("codesign")
                .arg("--force")
                .arg("-s")
                .arg("-")
                .arg("--timestamp=none")
                .arg(&app_path)
                .output()
                .map_err(|e| format!("重新签名失败：{}", e))?;

            if !output.status.success() {
                return Err(format!("重新签名失败：{:?}", String::from_utf8_lossy(&output.stderr)));
            }

            steps.push("应用签名完成".to_string());
        }
    } else if let Some(icon_path) = new_icon {
        steps.push(format!("步骤 1/2: 转换并替换应用图标：{}", icon_path));
        let icon_source = Path::new(&icon_path);
        let icon_dest = app_path.join("Contents/Resources/AppIcon.icns");

        if icon_source.exists() {
            convert_png_to_icns(&icon_path, &icon_dest.to_string_lossy())?;
            steps.push("图标格式已转换为.icns".to_string());
            
            let plist_path = app_path.join("Contents/Info.plist");
            let _ = Command::new("/usr/libexec/PlistBuddy")
                .arg("-c")
                .arg("Set :CFBundleIconFile AppIcon.icns")
                .arg(&plist_path)
                .output();
            steps.push("图标配置已更新".to_string());
        } else {
            steps.push("图标文件不存在，跳过".to_string());
        }
        
        let username = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
        
        let _ = Command::new("chown")
            .arg("-R")
            .arg(format!("{}:staff", username))
            .arg(&app_path)
            .output();

        let _ = Command::new("chmod")
            .arg("-R")
            .arg("755")
            .arg(&app_path)
            .output();
        
        steps.push("步骤 2/2: 重新签名应用".to_string());

        let username = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
        
        let output = Command::new("osascript")
            .arg("-e")
            .arg(format!(r#"do shell script "chown -R {}:staff '{}' && chmod -R 755 '{}'" with administrator privileges"#, 
                username, app_path.display(), app_path.display()))
            .output()
            .map_err(|e| format!("请求管理员权限失败：{}", e))?;

        if !output.status.success() {
            return Err("用户取消了权限请求".to_string());
        }

        let output = Command::new("codesign")
            .arg("--force")
            .arg("-s")
            .arg("-")
            .arg("--timestamp=none")
            .arg(&app_path)
            .output()
            .map_err(|e| {
                format!("重新签名失败：{}。\n\n请尝试以下方法：\n1. 在 Finder 中找到 /Applications/{}.app\n2. 右键点击 -> 显示简介\n3. 解锁（点击左下角锁图标）\n4. 确保你的用户有读写权限\n5. 或者在终端运行：sudo chown -R $USER:staff /Applications/{}.app", 
                    e, app_name, app_name)
            })?;

        if !output.status.success() {
            return Err(format!("重新签名失败：{:?}", String::from_utf8_lossy(&output.stderr)));
        }

        steps.push("应用签名完成".to_string());
    } else {
        return Err("至少需要修改名称或图标".to_string());
    }

    steps.push("✅ 更新完成！".to_string());

    Ok(steps.join("\n"))
}

#[tauri::command]
fn launch_dual_wechat(app_name: String) -> Result<String, String> {
    let app_path = format!("/Applications/{}.app", app_name);
    let app_path_obj = Path::new(&app_path);

    if !app_path_obj.exists() {
        return Err("分身应用不存在".to_string());
    }

    let output = Command::new("open")
        .arg("-n")
        .arg(&app_path)
        .output()
        .map_err(|e| format!("启动应用失败：{}", e))?;

    if !output.status.success() {
        return Err(format!("启动应用失败：{:?}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(format!("分身微信 {} 已启动", app_name))
}

fn fix_slash_in_bundle_id(app_path: &Path) -> Result<(), String> {
    let plist_path = app_path.join("Contents/Info.plist");
    
    if !plist_path.exists() {
        return Ok(());
    }

    let output = Command::new("/usr/libexec/PlistBuddy")
        .arg("-c")
        .arg("Print :CFBundleIdentifier")
        .arg(&plist_path)
        .output()
        .map_err(|e| format!("读取 Bundle ID 失败：{}", e))?;

    if !output.status.success() {
        return Ok(());
    }

    let bundle_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    if bundle_id.contains('/') {
        let fixed_bundle_id = bundle_id.replace('/', ".");
        
        let output = Command::new("/usr/libexec/PlistBuddy")
            .arg("-c")
            .arg(format!("Set :CFBundleIdentifier {}", fixed_bundle_id))
            .arg(&plist_path)
            .output()
            .map_err(|e| format!("修改 Bundle ID 失败：{}", e))?;

        if !output.status.success() {
            return Err(format!("修改 Bundle ID 失败：{:?}", String::from_utf8_lossy(&output.stderr)));
        }
    }

    Ok(())
}

#[tauri::command]
fn fix_icon_slash(app_name: String) -> Result<String, String> {
    let app_path = format!("/Applications/{}.app", app_name);
    let app_path_obj = Path::new(&app_path);

    if !app_path_obj.exists() {
        return Err("应用不存在".to_string());
    }

    let mut steps = Vec::new();
    steps.push("开始修复图标斜杠...".to_string());

    steps.push("步骤 1/4: 修复 Bundle ID 斜杠".to_string());
    fix_slash_in_bundle_id(app_path_obj)?;
    steps.push("Bundle ID 斜杠已修复".to_string());

    steps.push("步骤 2/4: 清除隔离属性".to_string());
    let _ = Command::new("xattr")
        .arg("-cr")
        .arg(&app_path)
        .output();
    steps.push("隔离属性已清除".to_string());

    steps.push("步骤 3/4: 修复文件权限".to_string());
    let username = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    
    let _ = Command::new("chown")
        .arg("-R")
        .arg(format!("{}:staff", username))
        .arg(&app_path)
        .output();

    let _ = Command::new("chmod")
        .arg("-R")
        .arg("755")
        .arg(&app_path)
        .output();

    steps.push("文件权限已修复".to_string());

    steps.push("步骤 4/4: 重新签名应用".to_string());
    let output = Command::new("codesign")
        .arg("--force")
        .arg("-s")
        .arg("-")
        .arg("--timestamp=none")
        .arg(&app_path)
        .output()
        .map_err(|e| format!("重新签名失败：{}", e))?;

    if !output.status.success() {
        return Err(format!("重新签名失败：{:?}", String::from_utf8_lossy(&output.stderr)));
    }

    steps.push("应用签名完成".to_string());
    steps.push("✅ 修复完成！".to_string());

    Ok(steps.join("\n"))
}

#[tauri::command]
fn get_dual_wechat_list() -> Result<Vec<String>, String> {
    let apps_dir = Path::new("/Applications");
    let mut dual_apps: Vec<String> = Vec::new();
    let mut seen_names: HashSet<String> = HashSet::new();

    if let Ok(entries) = fs::read_dir(apps_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if is_wechat_dual_app(&path) {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    if !seen_names.contains(name) {
                        seen_names.insert(name.to_string());
                        dual_apps.push(name.to_string());
                    }
                }
            }
        }
    }

    dual_apps.sort();
    Ok(dual_apps)
}

#[tauri::command]
fn delete_dual_wechat(app_name: String) -> Result<String, String> {
    let app_path = format!("/Applications/{}.app", app_name);
    let app_path_obj = Path::new(&app_path);

    if !app_path_obj.exists() {
        return Err("应用不存在".to_string());
    }

    if let Some(container_path) = get_app_container_path(app_path_obj) {
        let marker_path = Path::new(&container_path).join(MARKER_FILE);
        let _ = fs::remove_file(marker_path);
    }

    fs::remove_dir_all(&app_path_obj)
        .map_err(|e| format!("删除应用失败：{}", e))?;

    Ok(format!("分身应用 {} 已删除", app_name))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            read_file_binary,
            write_file_binary,
            check_wechat_installed,
            create_dual_wechat,
            update_dual_wechat,
            launch_dual_wechat,
            fix_icon_slash,
            get_dual_wechat_list,
            delete_dual_wechat,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
