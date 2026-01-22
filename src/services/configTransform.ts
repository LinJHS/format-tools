/**
 * 配置转换服务
 * 将模板配置转换为 Pandoc 元数据
 */

import type { TemplateConfig, PandocMetadata } from '../types/templateConfig'

/**
 * 将模板配置转换为 Pandoc 元数据
 */
export function buildPandocMetadata(config: Partial<TemplateConfig>): PandocMetadata {
  const metadata: PandocMetadata = {}
  
  // ==================== 基础元数据 ====================
  if (config.title) {
    metadata.title = config.title
    metadata['title-meta'] = config.title
  }
  
  if (config.author) {
    metadata.author = config.author
    // 处理多个作者的情况
    if (Array.isArray(config.author)) {
      metadata['author-meta'] = config.author.join(', ')
    } else {
      metadata['author-meta'] = config.author
    }
  }
  
  if (config.date) {
    metadata.date = config.date
    metadata['date-meta'] = config.date
  }
  
  if (config.subtitle) {
    metadata.subtitle = config.subtitle
  }
  
  if (config.abstract) {
    metadata.abstract = config.abstract
  }
  
  // ==================== 语言风格 ====================
  switch (config.languageStyle) {
    case 'zh-academic':
      metadata.figureTitle = '图'
      metadata.tableTitle = '表'
      metadata.listingTitle = '代码'
      metadata.figPrefix = '图'
      metadata.tblPrefix = '表'
      metadata.lstPrefix = '代码'
      metadata.eqnPrefix = '公式'
      metadata.secPrefix = '§'
      metadata.titleDelim = ':'
      break
      
    case 'en-academic':
      metadata.figureTitle = 'Figure'
      metadata.tableTitle = 'Table'
      metadata.listingTitle = 'Listing'
      metadata.figPrefix = ['fig.', 'figs.']
      metadata.tblPrefix = ['tbl.', 'tbls.']
      metadata.lstPrefix = ['lst.', 'lsts.']
      metadata.eqnPrefix = ['eq.', 'eqns.']
      metadata.secPrefix = ['sec.', 'secs.']
      metadata.titleDelim = ':'
      break
      
    case 'business':
      metadata.figureTitle = '图'
      metadata.tableTitle = '表'
      metadata.listingTitle = '代码'
      metadata.figPrefix = '图'
      metadata.tblPrefix = '表'
      metadata.lstPrefix = '代码'
      metadata.eqnPrefix = '公式'
      metadata.secPrefix = '章节'
      metadata.titleDelim = ' -'
      metadata.linkReferences = true
      metadata.nameInLink = true
      break
  }
  
  // ==================== 章节编号 ====================
  switch (config.sectionNumbering) {
    case 'none':
      metadata.chapters = false
      metadata.numberSections = false
      break
      
    case 'basic':
      metadata.chapters = true
      metadata.numberSections = true
      metadata.chaptersDepth = 1
      metadata.autoSectionLabels = true
      metadata.secHeaderDelim = ['.', '']
      break
      
    case 'from-h2':
      metadata.chapters = true
      metadata.numberSections = true
      metadata.chaptersDepth = 0  // 一级标题不编号
      metadata.sectionsDepth = 3
      metadata.autoSectionLabels = true
      metadata.secHeaderDelim = ['.', '']
      break
      
    case 'multilevel':
      metadata.chapters = true
      metadata.numberSections = true
      metadata.sectionsDepth = 4
      metadata.autoSectionLabels = true
      metadata.secHeaderDelim = ['.', '']
      break
  }
  
  // ==================== 交叉引用 ====================
  switch (config.crossReference) {
    case 'basic':
      metadata.linkReferences = false
      metadata.cref = false
      break
      
    case 'smart':
      metadata.linkReferences = true
      metadata.cref = true
      metadata.nameInLink = false
      break
      
    case 'full-link':
      metadata.linkReferences = true
      metadata.nameInLink = true
      metadata.cref = false
      break
  }
  
  // ==================== 公式编号 ====================
  switch (config.equationNumbering) {
    case 'manual':
      metadata.autoEqnLabels = false
      metadata.tableEqns = false
      break
      
    case 'auto':
      metadata.autoEqnLabels = true
      metadata.tableEqns = false
      break
      
    case 'table':
      metadata.autoEqnLabels = true
      metadata.tableEqns = true
      break
  }
  
  // ==================== 代码块 ====================
  switch (config.codeBlock) {
    case 'normal':
      metadata.listings = false
      metadata.codeBlockCaptions = true
      break
      
    case 'listings':
      metadata.listings = true
      metadata.codeBlockCaptions = true
      break
  }
  
  // ==================== 通用设置 ====================
  metadata.rangeDelim = '-'
  metadata.pairDelim = ', '
  metadata.lastDelim = ' 和 '
  metadata.refDelim = ', '
  metadata.chapDelim = '.'
  
  return metadata
}

/**
 * 合并配置（用户配置 > 模板预设 > 默认配置）
 */
export function mergeConfigs(
  userConfig: Partial<TemplateConfig>,
  templatePreset: Partial<TemplateConfig>,
  defaultConfig: Partial<TemplateConfig>
): TemplateConfig {
  return {
    // 合并顺序：用户 > 模板 > 默认
    ...defaultConfig,
    ...templatePreset,
    ...userConfig
  } as TemplateConfig
}

/**
 * 验证配置有效性
 */
export function validateConfig(config: Partial<TemplateConfig>): { valid: boolean, errors: string[] } {
  const errors: string[] = []
  
  // 验证日期格式（如果提供）
  if (config.date) {
    const dateRegex = /^\d{4}-\d{2}-\d{2}$/
    if (!dateRegex.test(config.date)) {
      errors.push('日期格式应为 YYYY-MM-DD')
    }
  }
  
  // 验证枚举值
  const validLanguageStyles = ['zh-academic', 'en-academic', 'business']
  if (config.languageStyle && !validLanguageStyles.includes(config.languageStyle)) {
    errors.push(`语言风格无效: ${config.languageStyle}`)
  }
  
  const validSectionNumbering = ['none', 'basic', 'from-h2', 'multilevel']
  if (config.sectionNumbering && !validSectionNumbering.includes(config.sectionNumbering)) {
    errors.push(`章节编号无效: ${config.sectionNumbering}`)
  }
  
  const validCrossReference = ['basic', 'smart', 'full-link']
  if (config.crossReference && !validCrossReference.includes(config.crossReference)) {
    errors.push(`交叉引用无效: ${config.crossReference}`)
  }
  
  const validEquationNumbering = ['manual', 'auto', 'table']
  if (config.equationNumbering && !validEquationNumbering.includes(config.equationNumbering)) {
    errors.push(`公式编号无效: ${config.equationNumbering}`)
  }
  
  const validCodeBlock = ['normal', 'listings']
  if (config.codeBlock && !validCodeBlock.includes(config.codeBlock)) {
    errors.push(`代码块无效: ${config.codeBlock}`)
  }
  
  return {
    valid: errors.length === 0,
    errors
  }
}
