import { onMounted, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import { Menu, MenuItem, PredefinedMenuItem } from '@tauri-apps/api/menu';

export function useContextMenu() {
  const router = useRouter();

  const handleContextMenu = async (e: MouseEvent) => {
    e.preventDefault();

    const target = e.target as HTMLElement;
    const isInput = target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable;
    const selection = window.getSelection();
    const hasSelection = selection ? selection.toString().length > 0 : false;

    let items = [];

    if (isInput || hasSelection) {
      // Input/Text Area Menu
      items = [
        await PredefinedMenuItem.new({ item: 'Cut', text: '剪切' }),
        await PredefinedMenuItem.new({ item: 'Copy', text: '复制' }),
        await PredefinedMenuItem.new({ item: 'Paste', text: '粘贴' }),
        await PredefinedMenuItem.new({ item: 'SelectAll', text: '全选' }),
      ];
    } else {
      // General Area Menu
      const refreshItem = await MenuItem.new({
        text: '刷新',
        action: () => {
          window.location.reload();
        },
      });

      const backItem = await MenuItem.new({
        text: '后退',
        enabled: window.history.length > 1,
        action: () => {
          window.history.back();
        },
      });

      const aboutItem = await MenuItem.new({
        text: '关于',
        action: () => {
          router.push('/about');
        },
      });

      items = [refreshItem, backItem, aboutItem];
    }

    const menu = await Menu.new({ items });
    await menu.popup();
  };

  onMounted(() => {
    document.addEventListener('contextmenu', handleContextMenu);
  });

  onUnmounted(() => {
    document.removeEventListener('contextmenu', handleContextMenu);
  });
}
