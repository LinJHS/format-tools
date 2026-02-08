import { ref } from 'vue';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { ask, message } from '@tauri-apps/plugin-dialog';

const updateStatus = ref('');
const progress = ref(0);
const totalSize = ref(0);
const isUpdating = ref(false);

export function useUpdate() {
  async function checkForUpdates(silent = false) {
    if (isUpdating.value) return;

    if (!silent) {
        updateStatus.value = '正在检查更新...';
    }
    
    try {
      const update = await check();
      
      if (update) {
        console.log(`发现新版本: ${update.version}`);
        console.log(`更新日志: ${update.body}`);
        
        const confirm = await ask(
          `发现新版本 v${update.version}\n\n${update.body || '暂无更新日志'}\n\n是否立即更新？`, 
          { title: 'Tauri 更新', kind: 'info', okLabel: '更新', cancelLabel: '取消' }
        );
  
        if (confirm) {
          isUpdating.value = true;
          updateStatus.value = '准备下载...';
          // 重置进度状态
          progress.value = 0;
          totalSize.value = 0;
          
          await update.downloadAndInstall((event) => {
            switch (event.event) {
              case 'Started':
                totalSize.value = event.data.contentLength || 0;
                updateStatus.value = '开始下载...';
                break;
              case 'Progress':
                // 计算进度百分比
                progress.value += event.data.chunkLength;
                if (totalSize.value > 0) {
                  updateStatus.value = `下载中: ${((progress.value / totalSize.value) * 100).toFixed(0)}%`;
                } else {
                   updateStatus.value = `已下载: ${(progress.value / 1024 / 1024).toFixed(2)} MB`;
                }
                break;
              case 'Finished':
                updateStatus.value = '下载完成，正在安装...';
                break;
            }
          });
  
          updateStatus.value = '更新完成，即将重启...';
          await new Promise(r => setTimeout(r, 1000));
          await relaunch();
        } else {
          if (!silent) updateStatus.value = '用户取消更新';
        }
      } else {
        if (!silent) {
            updateStatus.value = '当前已是最新版本';
            await message('当前已是最新版本', { title: '检查更新', kind: 'info' });
        }
      }
    } catch (error) {
      console.error(error);
      if (!silent) {
          updateStatus.value = `检查更新失败`;
          await message(`检查更新失败: 请检查您的网络连接，或联系客服解决`, { title: '错误', kind: 'error' });
      }
    } finally {
      isUpdating.value = false;
      if (silent && !isUpdating.value) {
          updateStatus.value = '';
      }
    }
  }

  return {
    updateStatus,
    progress,
    totalSize,
    isUpdating,
    checkForUpdates
  }
}
