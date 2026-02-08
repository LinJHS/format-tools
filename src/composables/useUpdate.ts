import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { ask, message } from '@tauri-apps/plugin-dialog';
import { sendNotification } from '@tauri-apps/plugin-notification';

export async function checkForAppUpdate(silent = false) {
  try {
    const update = await check();
    
    if (update) {
      // 1. 询问用户是否更新
      const shouldUpdate = await ask(
        `发现新版本 ${update.version}\n\n更新内容:\n${update.body || '暂无说明'}`, 
        {
          title: '发现新版本',
          kind: 'info',
          okLabel: '立即更新',
          cancelLabel: '稍后'
        }
      );

      if (shouldUpdate) {
        // 2. 发送通知：开始下载
        sendNotification({
          title: '正在更新',
          body: '更新包正在后台下载中，请稍候...'
        });

        // 3. 开始下载 (不监听详细进度，只等待完成)
        await update.downloadAndInstall();

        // 4. 下载完成，提示重启
        // 方式 A: 再次发送通知
        sendNotification({
          title: '更新完成',
          body: '下载已完成，应用即将重启以应用更新。'
        });
        
        // 方式 B: 弹窗确认 (更稳妥，防止用户没看到通知)
        await message('更新已下载完成，应用将立即重启。', { title: '更新完成', kind: 'info' });
        
        await relaunch();
      }
    } else {
      if (!silent) {
        await message('当前已是最新版本', { title: '检查更新', kind: 'info' });
      }
    }
  } catch (error) {
    console.error('更新失败:', error);
    if (!silent) {
      await message(`检查更新出错: ${error}`, { title: '错误', kind: 'error' });
    }
  }
}
