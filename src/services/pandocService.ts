import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export interface ConvertOptions {
  input_file: string
  output_file?: string
  source_dir?: string
  source_name?: string
  reference_doc?: string
  metadata?: Record<string, any>  // Pandoc 元数据对象
  metadata_file?: string
  use_crossref: boolean
}

export type InputSourceType = 'file' | 'text'

export interface PrepareInputPayload {
  source_type: InputSourceType
  path?: string
  original_name?: string
  selected_markdown?: string
  content?: string
  suggested_name?: string
}

export interface PreparedInput {
  markdown_path: string
  assets_dir: string
  image_count: number
  copied_images: string[]
  markdown_files: string[]
  source_name?: string
  source_dir?: string
}

export interface TemplateInfo {
  reference_doc: string
  protected_path: string
}

import type { TemplateConfig } from '../types/templateConfig'

export interface TemplateMeta {
  id: string
  name: string
  description: string
  category: string
  member: boolean
  defaultPreset?: Partial<TemplateConfig>
}

export interface TemplateListResponse {
  templates: TemplateMeta[]
  has_premium: boolean
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
  },

  /**
   * 预处理输入，提取 Markdown 及图片到临时目录
   */
  async prepareInput(payload: PrepareInputPayload): Promise<PreparedInput> {
    return await invoke<PreparedInput>('prepare_input_payload', { source: payload })
  },

  /**
   * 准备模板，返回可用的运行时路径
   */
  async prepareTemplate(templateName: string): Promise<TemplateInfo> {
    // Tauri v2 maps snake_case to camelCase in command args; Rust expects `templateName`.
    return await invoke<TemplateInfo>('prepare_template_protected', { templateName })
  },

  /**
   * 获取模板列表元数据
   */
  async getTemplates(): Promise<TemplateListResponse> {
    return await invoke<TemplateListResponse>('list_templates')
  }
}
