/**
 * 文件处理工具
 * 用于处理文件上传和转换
 */
import { error as logError } from '@tauri-apps/plugin-log'

export interface FileInfo {
  name: string
  size: number
  type: string
  content?: string
}

/**
 * 验证文件类型
 */
export const validateFileType = (file: File): boolean => {
  const allowedExtensions = ['.txt', '.md', '.zip', '.rar', '.7z']
  const fileName = file.name.toLowerCase()
  return allowedExtensions.some(ext => fileName.endsWith(ext))
}

/**
 * 获取文件信息
 */
export const getFileInfo = (file: File): FileInfo => {
  return {
    name: file.name,
    size: file.size,
    type: file.type
  }
}

/**
 * 检测是否为 Markdown 文件
 */
export const isMdFile = (fileName: string): boolean => {
  return fileName.toLowerCase().endsWith('.md')
}

/**
 * 检测是否为文本文件
 */
export const isTxtFile = (fileName: string): boolean => {
  return fileName.toLowerCase().endsWith('.txt')
}

/**
 * 检测是否为压缩文件
 */
export const isCompressedFile = (fileName: string): boolean => {
  const ext = fileName.toLowerCase()
  return ext.endsWith('.zip') || ext.endsWith('.rar') || ext.endsWith('.7z')
}

/**
 * 读取文件内容
 */
export const readFileContent = (file: File): Promise<string> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = (e) => {
      const content = e.target?.result as string
      resolve(content)
    }
    reader.onerror = () => {
      reject(new Error('Failed to read file'))
    }
    reader.readAsText(file)
  })
}

/**
 * 批量读取文件
 */
export const readMultipleFiles = async (files: File[]): Promise<FileInfo[]> => {
  const fileInfoList: FileInfo[] = []
  
  for (const file of files) {
    if (validateFileType(file)) {
      try {
        const content = await readFileContent(file)
        fileInfoList.push({
          ...getFileInfo(file),
          content
        })
      } catch (error) {
        logError(`Failed to read file: ${file.name}, error: ${error}`)
      }
    }
  }
  
  return fileInfoList
}

/**
 * 过滤 Markdown 文件
 */
export const filterMdFiles = (files: File[]): File[] => {
  return files.filter(file => isMdFile(file.name))
}

/**
 * 过滤文本文件
 */
export const filterTxtFiles = (files: File[]): File[] => {
  return files.filter(file => isTxtFile(file.name))
}
