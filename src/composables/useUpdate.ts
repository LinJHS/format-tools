import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { ask, message } from '@tauri-apps/plugin-dialog';
import { sendNotification, isPermissionGranted, requestPermission } from '@tauri-apps/plugin-notification';

export async function checkForAppUpdate(silent = false) {
  try {
    const update = await check();
    
    if (update) {
      // 1. 询问用户是否更新
      const shouldUpdate = await ask(
        `发现新版本 ${update.version}\n\n更新说明:\n${update.body || '暂无说明'}`, 
        {
          title: '格式匠 - 更新提示',
          kind: 'info',
          okLabel: '立即更新',
          cancelLabel: '稍后'
        }
      );

      if (shouldUpdate) {
        // 2. 发送通知：开始下载
        let permission = await isPermissionGranted();
        if (!permission) {
          permission = await requestPermission() === 'granted';
        }

        if (permission) {
          sendNotification({
            title: '格式匠 - 正在更新',
            body: '更新包正在后台下载中，请稍候...'
          });
        }

        // 3. 开始下载 (不监听详细进度，只等待完成)
        await update.download();

        // 4. 下载完成，提示重启
        // 弹窗确认
        await message('更新已下载完成，应用将立即安装并重启。', { title: '格式匠 - 下载完成', kind: 'info' });

        await update.install(); // 安装更新

        await relaunch();
      }
    } else {
      if (!silent) {
        await message('当前已是最新版本', { title: '格式匠 - 检查更新', kind: 'info' });
      }
    }
  } catch (error) {
    console.error('更新失败:', error);
    if (!silent) {
      await message(`检查更新出错，请检查您的网络连接或稍后重试。如有问题，请联系客服。`, { title: '格式匠 - 错误', kind: 'error' });
    }
  }
}
