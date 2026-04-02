const { invoke } = window.__TAURI__.core;
const { open } = window.__TAURI__.dialog;

let selectedIconPath = null;
let editIconPath = null;
let cropper = null;
let croppedImageData = null;
let pendingIconType = null;

function addLog(message, type = 'info') {
    const logOutput = document.getElementById('log-output');
    const logEntry = document.createElement('p');
    logEntry.className = `log-${type}`;
    logEntry.textContent = `[${new Date().toLocaleTimeString()}] ${message}`;
    logOutput.appendChild(logEntry);
    logOutput.scrollTop = logOutput.scrollHeight;
}

async function checkWeChatInstalled() {
    try {
        const installed = await invoke('check_wechat_installed');
        const statusIcon = document.querySelector('.status-icon');
        const statusText = document.getElementById('wechat-status-text');
        
        if (installed) {
            statusIcon.textContent = '✓';
            statusText.textContent = '微信已安装';
            addLog('微信已安装', 'success');
        } else {
            statusIcon.textContent = '✕';
            statusText.textContent = '微信未安装';
            addLog('微信未安装，请先安装微信', 'error');
        }
    } catch (error) {
        addLog(`检查微信安装状态失败：${error}`, 'error');
    }
}

async function loadDualApps() {
    try {
        const apps = await invoke('get_dual_wechat_list');
        const appsList = document.getElementById('dual-apps-list');
        
        if (apps.length === 0) {
            appsList.innerHTML = '<p class="empty-state">暂无分身应用</p>';
            return;
        }
        
        appsList.innerHTML = apps.map(app => `
            <div class="app-card">
                <div class="app-info">
                    <div class="app-name">${app}</div>
                    <div class="app-path">/Applications/${app}.app</div>
                </div>
                <div class="app-actions">
                    <button class="btn-launch" onclick="launchApp('${app}')">启动</button>
                    <button class="btn-edit" onclick="editApp('${app}')">编辑</button>
                    <button class="btn-fix" onclick="fixIconSlash('${app}')">修复</button>
                    <button class="btn-delete" onclick="deleteApp('${app}')">删除</button>
                </div>
            </div>
        `).join('');
        
        addLog(`检测到 ${apps.length} 个微信多开应用`, 'info');
    } catch (error) {
        addLog(`加载分身应用列表失败：${error}`, 'error');
    }
}

async function createDualWeChat(event) {
    event.preventDefault();
    
    const appName = document.getElementById('app-name').value.trim();
    const bundleId = document.getElementById('bundle-id').value.trim();
    const createBtn = document.getElementById('create-btn');
    
    if (!appName || !bundleId) {
        addLog('请填写应用名称和 Bundle ID', 'error');
        return;
    }
    
    createBtn.disabled = true;
    createBtn.textContent = '创建中...';
    
    try {
        addLog(`开始创建分身微信：${appName}`, 'info');
        const result = await invoke('create_dual_wechat', {
            appName,
            bundleId,
            customIcon: selectedIconPath
        });
        
        result.split('\n').forEach(line => {
            if (line) {
                addLog(line, 'success');
            }
        });
        
        addLog('分身微信创建成功！', 'success');
        await loadDualApps();
    } catch (error) {
        addLog(`创建分身微信失败：${error}`, 'error');
    } finally {
        createBtn.disabled = false;
        createBtn.textContent = '创建分身微信';
    }
}

async function launchApp(appName) {
    try {
        addLog(`正在启动 ${appName}...`, 'info');
        const result = await invoke('launch_dual_wechat', { appName });
        addLog(result, 'success');
    } catch (error) {
        addLog(`启动应用失败：${error}`, 'error');
    }
}

function editApp(appName) {
    editIconPath = null;
    document.getElementById('edit-app-name').value = appName;
    document.getElementById('edit-new-name').value = appName;
    document.getElementById('edit-selected-icon-name').textContent = '未选择';
    openModal();
}

function openModal() {
    const modal = document.getElementById('edit-modal');
    modal.classList.add('active');
}

function closeModal() {
    const modal = document.getElementById('edit-modal');
    modal.classList.remove('active');
}

async function updateApp(event) {
    event.preventDefault();
    
    const appName = document.getElementById('edit-app-name').value.trim();
    const newAppName = document.getElementById('edit-new-name').value.trim();
    const submitBtn = event.target.querySelector('button[type="submit"]');
    
    if (!newAppName) {
        addLog('请填写新应用名称', 'error');
        return;
    }
    
    submitBtn.disabled = true;
    submitBtn.textContent = '更新中...';
    
    try {
        addLog(`开始更新应用：${appName}`, 'info');
        const result = await invoke('update_dual_wechat', {
            appName,
            newAppName: newAppName !== appName ? newAppName : null,
            newIcon: editIconPath
        });
        
        result.split('\n').forEach(line => {
            if (line) {
                addLog(line, 'success');
            }
        });
        
        addLog('应用更新成功！', 'success');
        closeModal();
        await loadDualApps();
    } catch (error) {
        addLog(`更新应用失败：${error}`, 'error');
    } finally {
        submitBtn.disabled = false;
        submitBtn.textContent = '保存修改';
    }
}

async function fixIconSlash(appName) {
    try {
        addLog(`开始修复 ${appName} 的图标斜杠...`, 'info');
        const result = await invoke('fix_icon_slash', { appName });
        
        result.split('\n').forEach(line => {
            if (line) {
                addLog(line, 'success');
            }
        });
        
        addLog('图标斜杠修复完成！', 'success');
    } catch (error) {
        addLog(`修复图标斜杠失败：${error}`, 'error');
    }
}

async function deleteApp(appName) {
    if (!confirm(`确定要删除 ${appName} 吗？`)) {
        return;
    }
    
    try {
        addLog(`正在开始删除 ${appName}...`, 'info');
        const result = await invoke('delete_dual_wechat', { appName });
        addLog(result, 'success');
        await loadDualApps();
    } catch (error) {
        addLog(`删除应用失败：${error}`, 'error');
    }
}

async function selectIcon() {
    try {
        const selected = await open({
            multiple: false,
            filters: [
                {
                    name: '图标文件',
                    extensions: ['icns', 'png', 'jpg', 'jpeg']
                }
            ]
        });
        
        addLog(`选择结果：${JSON.stringify(selected)}`, 'info');
        
        if (selected) {
            pendingIconType = 'create';
            const fileName = selected.split('/').pop();
            document.getElementById('selected-icon-name').textContent = `已选择：${fileName}`;
            await showCropModal(selected);
        }
    } catch (error) {
        addLog(`选择图标失败：${error}`, 'error');
    }
}

async function selectEditIcon() {
    try {
        const selected = await open({
            multiple: false,
            filters: [
                {
                    name: '图标文件',
                    extensions: ['icns', 'png', 'jpg', 'jpeg']
                }
            ]
        });
        
        addLog(`选择结果：${JSON.stringify(selected)}`, 'info');
        
        if (selected) {
            pendingIconType = 'edit';
            const fileName = selected.split('/').pop();
            document.getElementById('edit-selected-icon-name').textContent = `已选择：${fileName}`;
            await showCropModal(selected);
        }
    } catch (error) {
        addLog(`选择图标失败：${error}`, 'error');
    }
}

async function showCropModal(imagePath) {
    try {
        const img = document.getElementById('crop-image');
        
        img.onload = null;
        img.onerror = null;
        img.src = '';
        
        const binaryData = await invoke('read_file_binary', { path: imagePath });
        
        const bytes = new Uint8Array(binaryData);
        let binary = '';
        for (let i = 0; i < bytes.length; i++) {
            binary += String.fromCharCode(bytes[i]);
        }
        const base64 = btoa(binary);
        
        const ext = imagePath.split('.').pop().toLowerCase();
        const mimeType = ext === 'png' ? 'image/png' : 'image/jpeg';
        const dataUrl = `data:${mimeType};base64,${base64}`;
        
        img.onload = () => {
            const cropModal = document.getElementById('crop-modal');
            cropModal.classList.add('active');
            
            if (cropper) {
                cropper.destroy();
            }
            
            setTimeout(() => {
                cropper = new window.Cropper(img, {
                    aspectRatio: 1,
                    viewMode: 1,
                    dragMode: 'move',
                    autoCropArea: 0.6,
                    restore: false,
                    guides: true,
                    center: true,
                    highlight: false,
                    cropBoxMovable: true,
                    cropBoxResizable: true,
                    toggleDragModeOnDblclick: false,
                    preview: '.preview-image',
                    responsive: true,
                });
                addLog('裁剪对话框已打开，请选择裁剪区域', 'info');
            }, 100);
            
            croppedImageData = null;
        };
        
        img.onerror = null;
        
        img.src = dataUrl;
        
    } catch (error) {
        addLog(`加载图片失败：${error}`, 'error');
    }
}

function closeCropModal() {
    const cropModal = document.getElementById('crop-modal');
    cropModal.classList.remove('active');
    
    if (cropper) {
        cropper.destroy();
        cropper = null;
    }
    
    const img = document.getElementById('crop-image');
    if (img) {
        img.onload = null;
        img.onerror = null;
        img.src = '';
    }
    
    croppedImageData = null;
    pendingIconType = null;
}

async function cropImage() {
    addLog('开始执行裁剪...', 'info');
    addLog(`当前类型：${pendingIconType}`, 'info');
    
    if (!cropper) {
        addLog('裁剪工具未初始化', 'error');
        return;
    }
    
    try {
        addLog('正在获取裁剪画布...', 'info');
        const canvas = cropper.getCroppedCanvas({
            width: 1024,
            height: 1024,
        });
        
        if (!canvas) {
            addLog('裁剪失败，无法获取画布', 'error');
            return;
        }
        
        addLog('画布获取成功，正在转换为 Blob...', 'info');
        canvas.toBlob(async (blob) => {
            try {
                if (!blob) {
                    addLog('裁剪失败，无法创建图片', 'error');
                    return;
                }
                
                addLog(`Blob 创建成功，大小：${blob.size} bytes`, 'info');
                
                const arrayBuffer = await blob.arrayBuffer();
                const binaryData = new Uint8Array(arrayBuffer);
                
                addLog('正在写入文件...', 'info');
                const tempDir = await window.__TAURI__.path.appDataDir();
                const timestamp = Date.now();
                const fileName = `cropped-icon-${timestamp}.png`;
                const filePath = await window.__TAURI__.path.join(tempDir, fileName);
                
                await invoke('write_file_binary', { path: filePath, data: Array.from(binaryData) });
                
                addLog(`文件已保存：${fileName}`, 'success');
                addLog(`完整路径：${filePath}`, 'info');
                
                if (pendingIconType === 'create') {
                    selectedIconPath = filePath;
                    const nameEl = document.getElementById('selected-icon-name');
                    if (nameEl) {
                        nameEl.textContent = `已裁剪`;
                        addLog(`图标已裁剪并保存：${fileName}`, 'success');
                    } else {
                        addLog('找不到 selected-icon-name 元素', 'error');
                    }
                } else if (pendingIconType === 'edit') {
                    editIconPath = filePath;
                    const nameEl = document.getElementById('edit-selected-icon-name');
                    if (nameEl) {
                        nameEl.textContent = `已裁剪`;
                        addLog(`图标已裁剪并保存：${fileName}`, 'success');
                    } else {
                        addLog('找不到 edit-selected-icon-name 元素', 'error');
                    }
                } else {
                    addLog(`未知的 pendingIconType: ${pendingIconType}`, 'error');
                }
                
                addLog('正在关闭裁剪对话框...', 'info');
                closeCropModal();
                addLog('裁剪完成！', 'success');
            } catch (innerError) {
                addLog(`处理裁剪结果失败：${innerError}`, 'error');
            }
        }, 'image/png', 1.0);
        
    } catch (error) {
        addLog(`裁剪图标失败：${error}`, 'error');
    }
}

function rotateImage() {
    if (!cropper) {
        addLog('裁剪工具未初始化', 'error');
        return;
    }
    cropper.rotate(90);
    addLog('图片已旋转 90°', 'info');
}

function resetImage() {
    if (!cropper) {
        addLog('裁剪工具未初始化', 'error');
        return;
    }
    cropper.reset();
    addLog('图片已重置', 'info');
}

function clearLog() {
    const logOutput = document.getElementById('log-output');
    logOutput.innerHTML = '<p class="log-info">日志已清空</p>';
}

document.addEventListener('DOMContentLoaded', () => {
    checkWeChatInstalled();
    loadDualApps();
    
    document.getElementById('create-form').addEventListener('submit', createDualWeChat);
    document.getElementById('select-icon-btn').addEventListener('click', selectIcon);
    document.getElementById('clear-log-btn').addEventListener('click', clearLog);
    
    document.getElementById('edit-form').addEventListener('submit', updateApp);
    document.getElementById('edit-select-icon-btn').addEventListener('click', selectEditIcon);
    document.getElementById('modal-close-btn').addEventListener('click', closeModal);
    document.getElementById('cancel-edit-btn').addEventListener('click', closeModal);
    document.querySelector('.modal-overlay').addEventListener('click', closeModal);
    
    document.getElementById('crop-modal-close-btn').addEventListener('click', closeCropModal);
    document.getElementById('crop-rotate-btn').addEventListener('click', rotateImage);
    document.getElementById('crop-reset-btn').addEventListener('click', resetImage);
    document.getElementById('crop-confirm-btn').addEventListener('click', cropImage);
    document.querySelectorAll('#crop-modal .modal-overlay')[0].addEventListener('click', closeCropModal);
    
    document.addEventListener('keydown', (e) => {
        if (e.key === 'Escape') {
            closeModal();
            closeCropModal();
        }
    });
    
    addLog('微信分身工具已启动', 'info');
    addLog('欢迎使用！', 'success');
});

window.launchApp = launchApp;
window.editApp = editApp;
window.fixIconSlash = fixIconSlash;
window.deleteApp = deleteApp;
