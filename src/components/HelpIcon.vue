<script setup lang="ts">
import { ref, computed } from 'vue'
import { OPTION_HELP } from '../types/templateConfig'
import type { OptionHelp } from '../types/templateConfig'

interface Props {
  field?: string
  group?: string
  option?: string
}

const props = defineProps<Props>()

const visible = ref(false)

const helpContent = computed<OptionHelp>(() => {
  // 字段级帮助
  if (props.field) {
    return getFieldHelp(props.field)
  }

  // 选项级帮助
  if (props.group && props.option) {
    return OPTION_HELP[props.group]?.[props.option] || getDefaultHelp()
  }

  // 组级帮助
  if (props.group) {
    return getGroupHelp(props.group)
  }

  return getDefaultHelp()
})

function getFieldHelp(field: string): OptionHelp {
  const fieldHelps: Record<string, OptionHelp> = {
    title: {
      title: '文档标题',
      description: '主标题，显示在文档最顶部。',
      example: '格式匠——一个专业的文档转换工具',
      affectedFields: ['title', 'title-meta'],
      preview: '无'
    },
    author: {
      title: '作者',
      description: '文档作者，可以是单个作者或多个作者（用逗号分隔）。',
      example: '格式匠, LinJHS',
      affectedFields: ['author', 'author-meta'],
      preview: '无'
    },
    date: {
      title: '日期',
      description: '文档创建或修改日期，格式为 YYYY/MM/DD。',
      example: '2026/01/20',
      affectedFields: ['date', 'date-meta'],
      preview: '日期: 2026年1月20日'
    },
    subtitle: {
      title: '副标题',
      description: '补充说明标题的子标题，可选。',
      example: '格式匠的原理与实现',
      affectedFields: ['subtitle'],
      preview: '无'
    },
    abstract: {
      title: '摘要',
      description: '文档内容的简短摘要，适合学术论文。',
      example: '本文提出了一种基于某算法的某方法...',
      affectedFields: ['abstract'],
      preview: '摘要: 本文提出了一种...'
    }
  }

  return fieldHelps[field] || getDefaultHelp()
}

function getGroupHelp(group: string): OptionHelp {
  const groupHelps: Record<string, OptionHelp> = {
    languageStyle: {
      title: '语言风格',
      description: '选择文档的语言和学术风格，影响图表、公式等元素的标题和引用格式。',
      example: '中文学术: "图 1: 系统架构"\n英文学术: "Figure 1: System Architecture"',
      affectedFields: ['figureTitle', 'tableTitle', 'figPrefix', 'tblPrefix', 'titleDelim'],
      preview: '点击单个选项查看详细效果'
    },
    sectionNumbering: {
      title: '章节编号',
      description: '控制章节标题的编号方式。',
      example: '无编号: "引言"\n基础编号: "1 引言"\n多级编号: "1.1.1 背景"',
      affectedFields: ['chapters', 'numberSections', 'chaptersDepth', 'sectionsDepth'],
      preview: '点击单个选项查看详细效果'
    },
    crossReference: {
      title: '交叉引用',
      description: '控制文档内部引用的显示方式和链接行为。',
      example: '基础: "见图 1"\n智能: "见图 1"（带链接）\n全链接: "见图 1"（全文本可点击）',
      affectedFields: ['linkReferences', 'cref', 'nameInLink'],
      preview: '点击单个选项查看详细效果'
    },
    equationNumbering: {
      title: '公式编号',
      description: '控制数学公式的编号方式。',
      example: '手动: 只有标记的公式编号\n自动: 所有公式自动编号',
      affectedFields: ['autoEqnLabels', 'tableEqns'],
      preview: '点击单个选项查看详细效果'
    },
    codeBlock: {
      title: '代码块',
      description: '选择代码块的渲染方式。',
      example: '普通: 标准代码高亮\nListings: LaTeX 专业排版',
      affectedFields: ['listings', 'codeBlockCaptions'],
      preview: '点击单个选项查看详细效果'
    }
  }

  return groupHelps[group] || getDefaultHelp()
}

function getDefaultHelp(): OptionHelp {
  return {
    title: '帮助',
    description: '暂无帮助信息',
    affectedFields: []
  }
}

function showHelp() {
  visible.value = true
}

function hideHelp() {
  visible.value = false
}
</script>

<template>
  <span class="help-icon" @click="showHelp">
    <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
      <circle cx="8" cy="8" r="7" stroke="currentColor" stroke-width="1" fill="none" />
      <text x="8" y="12.5" text-anchor="middle" font-size="12" font-weight="bold" fill="currentColor">?</text>
    </svg>
  </span>

  <teleport to="body">
    <div v-if="visible" class="help-modal-overlay" @click="hideHelp">
      <div class="help-modal" @click.stop>
        <div class="help-header">
          <h3>{{ helpContent.title }}</h3>
          <button class="close-btn" @click="hideHelp">&times;</button>
        </div>
        <div class="help-body">
          <p class="help-description" v-html="helpContent.description"></p>
        </div>
      </div>
    </div>
  </teleport>
</template>

<style scoped>
.help-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  color: #888;
  transition: color 0.2s;
}

.help-icon:hover {
  color: #4CAF50;
}

.help-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.2s;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }

  to {
    opacity: 1;
  }
}

.help-modal {
  background: white;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  max-width: 600px;
  width: 90%;
  max-height: 80vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  animation: slideUp 0.3s;
}

@keyframes slideUp {
  from {
    transform: translateY(20px);
    opacity: 0;
  }

  to {
    transform: translateY(0);
    opacity: 1;
  }
}

.help-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid #eee;
}

.help-header h3 {
  margin: 0;
  font-size: 1.3rem;
  color: #333;
}

.close-btn {
  background: none;
  border: none;
  font-size: 2rem;
  line-height: 1;
  color: #888;
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all 0.2s;
}

.close-btn:hover {
  background: #f5f5f5;
  color: #333;
}

.help-body {
  padding: 20px;
  overflow-y: auto;
}

.help-description {
  margin: 0 0 20px 0;
  font-size: 1rem;
  line-height: 1.6;
  color: #555;
}
</style>
