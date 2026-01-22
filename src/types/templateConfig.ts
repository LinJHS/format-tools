/**
 * 模板配置类型定义
 */

// ==================== 基础元数据配置 ====================

export interface BasicMetadata {
  title?: string
  author?: string | string[]
  date?: string
  subtitle?: string
  abstract?: string
}

// ==================== 高级配置选项 ====================

export type LanguageStyle = 'zh-academic' | 'en-academic' | 'business'
export type SectionNumbering = 'none' | 'basic' | 'from-h2' | 'custom'
export type CrossReference = 'basic' | 'full-link'
export type EquationNumbering = 'manual' | 'auto'

// 自定义章节编号配置
export interface CustomSectionConfig {
  startLevel: number  // 从几级标题开始编号 (1-6)
  depth: number       // 往下几级 (1-6)
}

export interface AdvancedConfig {
  languageStyle: LanguageStyle
  sectionNumbering: SectionNumbering
  customSectionConfig?: CustomSectionConfig  // 当 sectionNumbering = 'custom' 时使用
  crossReference: CrossReference
  equationNumbering: EquationNumbering
}

// ==================== 完整模板配置 ====================

export interface TemplateConfig extends BasicMetadata {
  // 高级配置
  languageStyle: LanguageStyle
  sectionNumbering: SectionNumbering
  customSectionConfig?: CustomSectionConfig
  crossReference: CrossReference
  equationNumbering: EquationNumbering
}

// ==================== 配置预设 ====================

export interface ConfigPreset {
  id: string
  name: string
  description?: string
  config: Partial<TemplateConfig>
  isBuiltin?: boolean  // 是否为内置预设
  createdAt?: string
}

// ==================== Pandoc 元数据映射 ====================

export interface PandocMetadata {
  // 基础元数据
  title?: string
  'title-meta'?: string
  author?: string | string[]
  'author-meta'?: string
  date?: string
  'date-meta'?: string
  subtitle?: string
  abstract?: string
  
  // pandoc-crossref 配置
  figureTitle?: string
  tableTitle?: string
  listingTitle?: string
  figPrefix?: string | string[]
  tblPrefix?: string | string[]
  lstPrefix?: string | string[]
  eqnPrefix?: string | string[]
  secPrefix?: string | string[]
  titleDelim?: string
  
  // 章节编号
  chapters?: boolean
  numberSections?: boolean
  chaptersDepth?: number
  sectionsDepth?: number
  autoSectionLabels?: boolean
  secHeaderDelim?: string | string[]
  
  // 交叉引用
  linkReferences?: boolean
  cref?: boolean
  nameInLink?: boolean
  
  // 公式编号
  autoEqnLabels?: boolean
  tableEqns?: boolean
  equationNumberTeX?: string
  
  // 代码块
  listings?: boolean
  codeBlockCaptions?: boolean
  
  // 其他选项
  rangeDelim?: string
  pairDelim?: string
  lastDelim?: string
  refDelim?: string
  chapDelim?: string
}

// ==================== 选项帮助信息 ====================

export interface OptionHelp {
  title: string
  description: string
  example?: string
  affectedFields: string[]
  preview?: string
}

export const OPTION_HELP: Record<string, Record<string, OptionHelp>> = {
  languageStyle: {
    'zh-academic': {
      title: '中文学术风格',
      description: '适用于中文学术论文、报告、书籍。使用"图"、"表"、"公式"等中文前缀。',
      example: '图题显示为: "图 1: 系统架构"\n引用显示为: "如图 1 所示..."',
      affectedFields: ['figureTitle', 'tableTitle', 'figPrefix', 'tblPrefix', 'titleDelim'],
      preview: '图 1: 系统架构图'
    },
    'en-academic': {
      title: '英文学术风格',
      description: '适用于英文学术论文、国际期刊投稿。使用"Figure"、"Table"等英文前缀，支持单复数。',
      example: '单个引用: "see fig. 1"\n多个引用: "see figs. 1-3"',
      affectedFields: ['figureTitle', 'tableTitle', 'figPrefix', 'tblPrefix'],
      preview: 'Figure 1: System Architecture'
    },
    'business': {
      title: '商务报告风格',
      description: '适用于企业报告、项目文档。使用清晰的分隔符，所有引用带超链接。',
      example: '图题显示为: "图 1 - 销售趋势"\n引用可点击跳转',
      affectedFields: ['titleDelim', 'linkReferences', 'nameInLink'],
      preview: '图 1 - 销售趋势图'
    }
  },
  sectionNumbering: {
    'none': {
      title: '无编号',
      description: '章节标题不带编号，适合短文档、博客文章。',
      example: '# 前言\n## 背景\n## 目标',
      affectedFields: ['chapters', 'numberSections'],
      preview: '前言\n  背景\n  目标'
    },
    'basic': {
      title: '从一级标题开始',
      description: '从一级标题(#)开始编号，最多往下三级标题。适合书籍、长文档。',
      example: '1 前言\n  1.1 背景\n    1.1.1 国内现状',
      affectedFields: ['chapters', 'numberSections', 'chaptersDepth', 'autoSectionLabels'],
      preview: '1 前言\n  1.1 背景\n    1.1.1 国内现状'
    },
    'from-h2': {
      title: '从二级标题开始',
      description: '一级标题作为章名不编号，从二级标题(##)开始编号，最多往下三级。',
      example: '引言（不编号）\n  1.1 研究背景\n    1.1.1 国内现状',
      affectedFields: ['chapters', 'numberSections', 'chaptersDepth', 'sectionsDepth'],
      preview: '引言\n  1.1 研究背景\n    1.1.1 国内现状'
    },
    'custom': {
      title: '自定义编号',
      description: '自定义从几级标题开始编号，以及往下编号几级。',
      example: '可选择从1-6级标题开始，往下1-6级',
      affectedFields: ['chapters', 'numberSections', 'sectionsDepth', 'chaptersDepth'],
      preview: '根据自定义配置生成'
    }
  },
  crossReference: {
    'basic': {
      title: '基础引用',
      description: '标准的交叉引用，不添加超链接。',
      example: '如图 1 所示',
      affectedFields: ['linkReferences', 'cref'],
      preview: '见图 1（纯文本）'
    },
    'full-link': {
      title: '全链接引用',
      description: '所有引用都带超链接，引用文本也包含在链接内。',
      example: '点击 "图 1" 整体跳转',
      affectedFields: ['linkReferences', 'nameInLink'],
      preview: '见图 1（全文本可点击）'
    }
  },
  equationNumbering: {
    'manual': {
      title: '手动编号',
      description: '只对带标签的公式编号。需要在公式后添加 {#eq:label} 才会编号。',
      example: '$$ E = mc^2 $$ {#eq:einstein}',
      affectedFields: ['autoEqnLabels'],
      preview: '仅标记的公式有编号'
    },
    'auto': {
      title: '自动编号',
      description: '所有独立公式（$$ ... $$）自动编号。',
      example: '所有 display math 自动获得编号',
      affectedFields: ['autoEqnLabels', 'tableEqns'],
      preview: '所有公式自动编号'
    }
  }
}

// ==================== 默认配置 ====================

export const DEFAULT_CONFIG: TemplateConfig = {
  title: '',
  author: '',
  date: new Date().toISOString().split('T')[0],  // 当前日期
  subtitle: '',
  abstract: '',
  languageStyle: 'zh-academic',
  sectionNumbering: 'none',
  crossReference: 'basic',
  equationNumbering: 'manual'
}

// ==================== 内置预设 ====================

export const BUILTIN_PRESETS: ConfigPreset[] = [
  {
    id: 'empty',
    name: '空预设',
    description: '所有选项使用默认值',
    config: {},
    isBuiltin: true
  },
  {
    id: 'zh-paper',
    name: '中文学术论文',
    description: '适合中文期刊投稿、学位论文',
    config: {
      languageStyle: 'zh-academic',
      sectionNumbering: 'from-h2',
      crossReference: 'basic',
      equationNumbering: 'auto'
    },
    isBuiltin: true
  },
  {
    id: 'en-paper',
    name: '英文学术论文',
    description: '适合国际期刊投稿',
    config: {
      languageStyle: 'en-academic',
      sectionNumbering: 'basic',
      crossReference: 'basic',
      equationNumbering: 'auto'
    },
    isBuiltin: true
  },
  {
    id: 'business',
    name: '商务报告',
    description: '适合企业报告、项目文档',
    config: {
      languageStyle: 'business',
      sectionNumbering: 'basic',
      crossReference: 'full-link',
      equationNumbering: 'manual'
    },
    isBuiltin: true
  },
  {
    id: 'technical',
    name: '技术文档',
    description: '适合技术手册、API 文档',
    config: {
      languageStyle: 'zh-academic',
      sectionNumbering: 'basic',  // 修改为 basic，自动最多三级
      crossReference: 'full-link',
      equationNumbering: 'manual'
    },
    isBuiltin: true
  }
]
