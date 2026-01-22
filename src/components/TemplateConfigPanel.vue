<script setup lang="ts">
import { ref, watch } from 'vue'
import type { TemplateConfig } from '../types/templateConfig'
import HelpIcon from './HelpIcon.vue'

interface Props {
    config: Partial<TemplateConfig>
    templatePreset?: Partial<TemplateConfig>
}

interface Emits {
    (e: 'update:config', value: Partial<TemplateConfig>): void
    (e: 'load-preset'): void
    (e: 'save-preset'): void
    (e: 'reset'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const localConfig = ref<Partial<TemplateConfig>>({ ...props.config })

// 监听本地配置变化，向上传递
watch(localConfig, (newConfig) => {
    emit('update:config', newConfig)
}, { deep: true })

// 监听外部配置变化，更新本地状态
watch(() => props.config, (newConfig) => {
    localConfig.value = { ...newConfig }
}, { deep: true })

function loadPreset() {
    emit('load-preset')
}

function saveAsPreset() {
    emit('save-preset')
}

function resetToTemplate() {
    emit('reset')
}
</script>


<template>
  <div class="template-config-panel">
    <h3>模板配置</h3>
    
    <!-- 基础元数据 -->
    <div class="config-section">
      <h4>基础信息</h4>
      <div class="form-group">
        <label>
          标题
          <HelpIcon field="title" />
        </label>
        <input 
          v-model="localConfig.title" 
          type="text" 
          placeholder="文档标题"
        />
      </div>
      
      <div class="form-group">
        <label>
          作者
          <HelpIcon field="author" />
        </label>
        <input 
          v-model="localConfig.author" 
          type="text" 
          placeholder="多个作者用逗号分隔"
        />
      </div>
      
      <div class="form-group">
        <label>
          日期
          <HelpIcon field="date" />
        </label>
        <input 
          v-model="localConfig.date" 
          type="date"
        />
      </div>
      
      <div class="form-group">
        <label>
          副标题
          <HelpIcon field="subtitle" />
        </label>
        <input 
          v-model="localConfig.subtitle" 
          type="text" 
          placeholder="可选"
        />
      </div>
      
      <div class="form-group">
        <label>
          摘要
          <HelpIcon field="abstract" />
        </label>
        <textarea 
          v-model="localConfig.abstract" 
          placeholder="可选，简短描述文档内容"
          rows="3"
        ></textarea>
      </div>
    </div>
    
    <!-- 高级配置 -->
    <div class="config-section">
      <h4>格式预设</h4>
      
      <!-- 语言风格 -->
      <div class="form-group">
        <label class="section-label">
          语言风格
          <HelpIcon group="languageStyle" />
        </label>
        <div class="radio-group">
          <label class="radio-option">
            <input 
              v-model="localConfig.languageStyle" 
              type="radio" 
              value="zh-academic"
            />
            <span>中文学术</span>
            <HelpIcon group="languageStyle" option="zh-academic" />
          </label>
          <label class="radio-option">
            <input 
              v-model="localConfig.languageStyle" 
              type="radio" 
              value="en-academic"
            />
            <span>英文学术</span>
            <HelpIcon group="languageStyle" option="en-academic" />
          </label>
          <label class="radio-option">
            <input 
              v-model="localConfig.languageStyle" 
              type="radio" 
              value="business"
            />
            <span>商务报告</span>
            <HelpIcon group="languageStyle" option="business" />
          </label>
        </div>
      </div>
      
      <!-- 章节编号 -->
      <div class="form-group">
        <label class="section-label">
          章节编号
          <HelpIcon group="sectionNumbering" />
        </label>
        <div class="radio-group">
          <label class="radio-option">
            <input 
              v-model="localConfig.sectionNumbering" 
              type="radio" 
              value="none"
            />
            <span>无编号</span>
            <HelpIcon group="sectionNumbering" option="none" />
          </label>
          <label class="radio-option">
            <input 
              v-model="localConfig.sectionNumbering" 
              type="radio" 
              value="basic"
            />
            <span>从一级标题</span>
            <HelpIcon group="sectionNumbering" option="basic" />
          </label>
          <label class="radio-option">
            <input 
              v-model="localConfig.sectionNumbering" 
              type="radio" 
              value="from-h2"
            />
            <span>从二级标题</span>
            <HelpIcon group="sectionNumbering" option="from-h2" />
          </label>
          <label class="radio-option">
            <input 
              v-model="localConfig.sectionNumbering" 
              type="radio" 
              value="multilevel"
            />
            <span>多级编号</span>
            <HelpIcon group="sectionNumbering" option="multilevel" />
          </label>
        </div>
      </div>
      
      <!-- 交叉引用 -->
      <div class="form-group">
        <label class="section-label">
          交叉引用
          <HelpIcon group="crossReference" />
        </label>
        <div class="radio-group">
          <label class="radio-option">
            <input 
              v-model="localConfig.crossReference" 
              type="radio" 
              value="basic"
            />
            <span>基础引用</span>
            <HelpIcon group="crossReference" option="basic" />
          </label>
          <label class="radio-option">
            <input 
              v-model="localConfig.crossReference" 
              type="radio" 
              value="smart"
            />
            <span>智能引用</span>
            <HelpIcon group="crossReference" option="smart" />
          </label>
          <label class="radio-option">
            <input 
              v-model="localConfig.crossReference" 
              type="radio" 
              value="full-link"
            />
            <span>全链接引用</span>
            <HelpIcon group="crossReference" option="full-link" />
          </label>
        </div>
      </div>
      
      <!-- 公式编号 -->
      <div class="form-group">
        <label class="section-label">
          公式编号
          <HelpIcon group="equationNumbering" />
        </label>
        <div class="radio-group">
          <label class="radio-option">
            <input 
              v-model="localConfig.equationNumbering" 
              type="radio" 
              value="manual"
            />
            <span>手动编号</span>
            <HelpIcon group="equationNumbering" option="manual" />
          </label>
          <label class="radio-option">
            <input 
              v-model="localConfig.equationNumbering" 
              type="radio" 
              value="auto"
            />
            <span>自动编号</span>
            <HelpIcon group="equationNumbering" option="auto" />
          </label>
          <label class="radio-option">
            <input 
              v-model="localConfig.equationNumbering" 
              type="radio" 
              value="table"
            />
            <span>表格式编号</span>
            <HelpIcon group="equationNumbering" option="table" />
          </label>
        </div>
      </div>
      
      <!-- 代码块 -->
      <div class="form-group">
        <label class="section-label">
          代码块
          <HelpIcon group="codeBlock" />
        </label>
        <div class="radio-group">
          <label class="radio-option">
            <input 
              v-model="localConfig.codeBlock" 
              type="radio" 
              value="normal"
            />
            <span>普通代码块</span>
            <HelpIcon group="codeBlock" option="normal" />
          </label>
          <label class="radio-option">
            <input 
              v-model="localConfig.codeBlock" 
              type="radio" 
              value="listings"
            />
            <span>LaTeX listings</span>
            <HelpIcon group="codeBlock" option="listings" />
          </label>
        </div>
      </div>
    </div>
    
    <!-- 预设管理 -->
    <div class="config-section">
      <h4>预设管理</h4>
      <div class="preset-actions">
        <button @click="loadPreset" class="btn-secondary">
          加载预设
        </button>
        <button @click="saveAsPreset" class="btn-secondary">
          保存为预设
        </button>
        <button @click="resetToTemplate" class="btn-secondary">
          重置为模板默认
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.template-config-panel {
  padding: 20px;
  background: #f9f9f9;
  border-radius: 8px;
}

h3 {
  margin: 0 0 20px 0;
  font-size: 1.5rem;
  color: #333;
}

h4 {
  margin: 0 0 15px 0;
  font-size: 1.2rem;
  color: #555;
}

.config-section {
  margin-bottom: 30px;
  padding: 20px;
  background: white;
  border-radius: 6px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.form-group {
  margin-bottom: 20px;
}

.form-group:last-child {
  margin-bottom: 0;
}

label {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
  font-weight: 500;
  color: #333;
}

.section-label {
  font-size: 1.05rem;
  color: #222;
}

input[type="text"],
input[type="date"],
textarea {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  transition: border-color 0.2s;
}

input[type="text"]:focus,
input[type="date"]:focus,
textarea:focus {
  outline: none;
  border-color: #4CAF50;
}

textarea {
  resize: vertical;
  font-family: inherit;
}

.radio-group {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.radio-option {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.radio-option:hover {
  background: #f5f5f5;
  border-color: #bbb;
}

.radio-option input[type="radio"] {
  cursor: pointer;
}

.radio-option input[type="radio"]:checked + span {
  font-weight: 600;
  color: #4CAF50;
}

.preset-actions {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.btn-secondary {
  padding: 8px 16px;
  background: white;
  border: 1px solid #ddd;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.btn-secondary:hover {
  background: #f5f5f5;
  border-color: #bbb;
}

.btn-secondary:active {
  background: #e9e9e9;
}
</style>
