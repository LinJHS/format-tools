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
      description: '<p>适用于中文学术论文、报告、书籍，使用中文前缀。</p><pre>图 1：系统架构\n如图 1 所示...</pre>',
      affectedFields: ['figureTitle', 'tableTitle', 'figPrefix', 'tblPrefix', 'titleDelim'],
      preview: '图 1: 系统架构图'
    },
    'en-academic': {
      title: '英文学术风格',
      description: '<p>适用于英文学术论文、国际期刊，使用英文前缀。</p><pre>Figure 1: System Architecture\nsee fig. 1</pre>',
      affectedFields: ['figureTitle', 'tableTitle', 'figPrefix', 'tblPrefix'],
      preview: 'Figure 1: System Architecture'
    },
    'business': {
      title: '商务报告风格',
      description: '<p>适用于企业报告、项目文档，引用默认带链接。</p><pre>图 1 - 销售趋势\n点击“图 1”可跳转</pre>',
      affectedFields: ['titleDelim', 'linkReferences', 'nameInLink'],
      preview: '图 1 - 销售趋势图'
    }
  },
  sectionNumbering: {
    'none': {
      title: '无编号',
      description: '<p>章节标题不带编号，适合短文档、博客。</p><pre># 前言\n## 背景\n## 目标</pre>',
      affectedFields: ['chapters', 'numberSections'],
      preview: '前言\n  背景\n  目标'
    },
    'basic': {
      title: '从一级标题开始',
      description: '<p>从一级标题(#)开始编号，默认向下 3 级。</p><pre>1 前言\n  1.1 背景\n    1.1.1 国内现状</pre>',
      affectedFields: ['chapters', 'numberSections', 'chaptersDepth', 'autoSectionLabels'],
      preview: '1 前言\n  1.1 背景\n    1.1.1 国内现状'
    },
    'from-h2': {
      title: '从二级标题开始',
      description: '<p>一级标题不编号，从二级标题(##)开始，默认向下 3 级。</p><pre>引言（不编号）\n  1.1 研究背景\n    1.1.1 国内现状</pre>',
      affectedFields: ['chapters', 'numberSections', 'chaptersDepth', 'sectionsDepth'],
      preview: '引言\n  1.1 研究背景\n    1.1.1 国内现状'
    },
    'custom': {
      title: '自定义编号',
      description: '<p>自定义起始级别和深度。</p><pre>起始：2级 (##)\n深度：4级</pre>',
      affectedFields: ['chapters', 'numberSections', 'sectionsDepth', 'chaptersDepth'],
      preview: '根据自定义配置生成'
    }
  },
  crossReference: {
    'basic': {
      title: '基础引用',
      description: '<p>标准交叉引用，不添加超链接。</p><pre>如图 1 所示</pre>',
      affectedFields: ['linkReferences', 'cref'],
      preview: '见图 1（纯文本）'
    },
    'full-link': {
      title: '全链接引用',
      description: '<p>所有引用带超链接，引用文本整体可点击。</p><pre>点击 “图 1” 整体跳转</pre>',
      affectedFields: ['linkReferences', 'nameInLink'],
      preview: '见图 1（全文本可点击）'
    }
  },
  equationNumbering: {
    'manual': {
      title: '手动编号',
      description: '<p>只对带标签的公式编号，需要 {#eq:label}。</p><pre>$$ E = mc^2 $$ {#eq:einstein}</pre>',
      affectedFields: ['autoEqnLabels'],
      preview: '仅标记的公式有编号'
    },
    'auto': {
      title: '自动编号',
      description: '<p>所有独立公式（$$...$$）自动编号。</p><pre>$$ a^2 + b^2 = c^2 $$</pre>',
      affectedFields: ['autoEqnLabels', 'tableEqns'],
      preview: '所有公式自动编号'
    }
  }
}

// ==================== 默认配置 ====================

export const DEFAULT_CONFIG: TemplateConfig = {
  title: '',
  author: '',
  date: '',
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
