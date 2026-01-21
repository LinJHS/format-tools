import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export interface ConvertOptions {
  input_file: string
  output_file: string
  reference_doc?: string
  metadata_file?: string
  use_crossref: boolean
}

export interface DownloadProgress {
  downloaded: number
  total: number
  percentage: number
}

export const pandocService = {
  /**
   * 检查 Pandoc 是否已安装
   */
  async isPandocInstalled(): Promise<boolean> {
    return await invoke('is_pandoc_installed')
  },

  /**
   * 检查 Pandoc-crossref 是否已安装
   */
  async isCrossrefInstalled(): Promise<boolean> {
    return await invoke('is_crossref_installed')
  },

  /**
   * 获取 Pandoc 版本
   */
  async getPandocVersion(): Promise<string> {
    return await invoke('pandoc_version')
  },

  /**
   * 安装 Pandoc
   */
  async installPandoc(onProgress?: (progress: DownloadProgress) => void): Promise<string> {
    if (onProgress) {
      const unlisten = await listen<DownloadProgress>('pandoc-download-progress', (event) => {
        onProgress(event.payload)
      })
      
      try {
        const result = await invoke<string>('install_pandoc')
        unlisten()
        return result
      } catch (error) {
        unlisten()
        throw error
      }
    } else {
      return await invoke<string>('install_pandoc')
    }
  },

  /**
   * 安装 Pandoc-crossref
   */
  async installCrossref(onProgress?: (progress: DownloadProgress) => void): Promise<string> {
    if (onProgress) {
      const unlisten = await listen<DownloadProgress>('crossref-download-progress', (event) => {
        onProgress(event.payload)
      })
      
      try {
        const result = await invoke<string>('install_crossref')
        unlisten()
        return result
      } catch (error) {
        unlisten()
        throw error
      }
    } else {
      return await invoke<string>('install_crossref')
    }
  },

  /**
   * 转换 Markdown 到 DOCX
   */
  async convertMarkdown(options: ConvertOptions): Promise<string> {
    return await invoke<string>('convert_markdown', { options })
  },

  /**
   * 确保 Pandoc 和 Crossref 已安装
   */
  async ensureInstalled(
    onPandocProgress?: (progress: DownloadProgress) => void,
    onCrossrefProgress?: (progress: DownloadProgress) => void
  ): Promise<void> {
    const pandocInstalled = await this.isPandocInstalled()
    if (!pandocInstalled) {
      await this.installPandoc(onPandocProgress)
    }

    const crossrefInstalled = await this.isCrossrefInstalled()
    if (!crossrefInstalled) {
      await this.installCrossref(onCrossrefProgress)
    }
  }
}
