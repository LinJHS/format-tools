/**
 * 配置本地存储服务
 * 管理用户配置历史记录和自定义预设
 */

import type { TemplateConfig, ConfigPreset } from '../types/templateConfig'

const STORAGE_KEYS = {
  RECENT_CONFIGS: 'template-recent-configs',
  CUSTOM_PRESETS: 'template-custom-presets'
}

const MAX_RECENT_CONFIGS = 3

// ==================== 历史记录管理 ====================

/**
 * 保存配置到历史记录
 */
export function saveRecentConfig(config: Partial<TemplateConfig>): void {
  try {
    const recent = getRecentConfigs()
    
    // 添加到列表开头
    recent.unshift({
      config,
      timestamp: new Date().toISOString()
    })
    
    // 只保留最近 3 个
    const trimmed = recent.slice(0, MAX_RECENT_CONFIGS)
    
    localStorage.setItem(STORAGE_KEYS.RECENT_CONFIGS, JSON.stringify(trimmed))
  } catch (error) {
    console.error('保存历史配置失败:', error)
  }
}

/**
 * 获取历史配置列表
 */
export function getRecentConfigs(): Array<{ config: Partial<TemplateConfig>, timestamp: string }> {
  try {
    const stored = localStorage.getItem(STORAGE_KEYS.RECENT_CONFIGS)
    if (!stored) return []
    
    return JSON.parse(stored)
  } catch (error) {
    console.error('读取历史配置失败:', error)
    return []
  }
}

/**
 * 清空历史配置
 */
export function clearRecentConfigs(): void {
  try {
    localStorage.removeItem(STORAGE_KEYS.RECENT_CONFIGS)
  } catch (error) {
    console.error('清空历史配置失败:', error)
  }
}

// ==================== 自定义预设管理 ====================

/**
 * 保存自定义预设
 */
export function saveCustomPreset(preset: Omit<ConfigPreset, 'id' | 'createdAt'>): ConfigPreset {
  try {
    const presets = getCustomPresets()
    
    // 生成唯一 ID
    const id = `custom-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`
    
    const newPreset: ConfigPreset = {
      ...preset,
      id,
      isBuiltin: false,
      createdAt: new Date().toISOString()
    }
    
    presets.push(newPreset)
    localStorage.setItem(STORAGE_KEYS.CUSTOM_PRESETS, JSON.stringify(presets))
    
    return newPreset
  } catch (error) {
    console.error('保存自定义预设失败:', error)
    throw error
  }
}

/**
 * 更新自定义预设
 */
export function updateCustomPreset(id: string, preset: Partial<ConfigPreset>): boolean {
  try {
    const presets = getCustomPresets()
    const index = presets.findIndex(p => p.id === id)
    
    if (index === -1) {
      console.warn(`预设不存在: ${id}`)
      return false
    }
    
    // 不允许修改内置预设
    if (presets[index].isBuiltin) {
      console.warn('不能修改内置预设')
      return false
    }
    
    // 更新预设
    presets[index] = {
      ...presets[index],
      ...preset,
      id,  // 保持 ID 不变
      isBuiltin: false  // 保持非内置标记
    }
    
    localStorage.setItem(STORAGE_KEYS.CUSTOM_PRESETS, JSON.stringify(presets))
    return true
  } catch (error) {
    console.error('更新自定义预设失败:', error)
    return false
  }
}

/**
 * 获取所有自定义预设
 */
export function getCustomPresets(): ConfigPreset[] {
  try {
    const stored = localStorage.getItem(STORAGE_KEYS.CUSTOM_PRESETS)
    if (!stored) return []
    
    return JSON.parse(stored)
  } catch (error) {
    console.error('读取自定义预设失败:', error)
    return []
  }
}

/**
 * 获取单个预设
 */
export function getCustomPreset(id: string): ConfigPreset | null {
  const presets = getCustomPresets()
  return presets.find(p => p.id === id) || null
}

/**
 * 删除自定义预设
 */
export function deleteCustomPreset(id: string): boolean {
  try {
    const presets = getCustomPresets()
    const index = presets.findIndex(p => p.id === id)
    
    if (index === -1) {
      console.warn(`预设不存在: ${id}`)
      return false
    }
    
    // 不允许删除内置预设
    if (presets[index].isBuiltin) {
      console.warn('不能删除内置预设')
      return false
    }
    
    presets.splice(index, 1)
    localStorage.setItem(STORAGE_KEYS.CUSTOM_PRESETS, JSON.stringify(presets))
    return true
  } catch (error) {
    console.error('删除自定义预设失败:', error)
    return false
  }
}

/**
 * 清空所有自定义预设
 */
export function clearCustomPresets(): void {
  try {
    localStorage.removeItem(STORAGE_KEYS.CUSTOM_PRESETS)
  } catch (error) {
    console.error('清空自定义预设失败:', error)
  }
}

// ==================== 导入导出 ====================

/**
 * 导出所有自定义预设为 JSON
 */
export function exportCustomPresets(): string {
  const presets = getCustomPresets()
  return JSON.stringify(presets, null, 2)
}

/**
 * 从 JSON 导入自定义预设
 */
export function importCustomPresets(json: string): { success: number, failed: number, errors: string[] } {
  const result = {
    success: 0,
    failed: 0,
    errors: [] as string[]
  }
  
  try {
    const imported = JSON.parse(json)
    
    if (!Array.isArray(imported)) {
      result.errors.push('导入数据必须是数组')
      return result
    }
    
    const existing = getCustomPresets()
    
    for (const preset of imported) {
      try {
        // 验证必需字段
        if (!preset.name || !preset.config) {
          result.failed++
          result.errors.push(`预设缺少必需字段: ${preset.name || '未命名'}`)
          continue
        }
        
        // 生成新 ID 避免冲突
        const newPreset: ConfigPreset = {
          ...preset,
          id: `custom-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
          isBuiltin: false,
          createdAt: new Date().toISOString()
        }
        
        existing.push(newPreset)
        result.success++
      } catch (error) {
        result.failed++
        result.errors.push(`导入预设失败: ${error}`)
      }
    }
    
    localStorage.setItem(STORAGE_KEYS.CUSTOM_PRESETS, JSON.stringify(existing))
  } catch (error) {
    result.errors.push(`JSON 解析失败: ${error}`)
  }
  
  return result
}
