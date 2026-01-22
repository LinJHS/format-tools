<template>
  <teleport to="body">
    <div v-if="visible" class="config-dialog-overlay" @click="close">
      <div class="config-dialog" @click.stop>
        <div class="dialog-header">
          <h3>配置选项</h3>
          <div class="header-actions">
            <button class="preset-btn" @click="loadPreset">加载预设</button>
            <button class="preset-btn" @click="saveAsPreset">保存预设</button>
            <button class="preset-btn" @click="resetToTemplate">重置</button>
            <button class="close-btn" @click="close">&times;</button>
          </div>
        </div>
        
        <div class="dialog-body">
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
                    value="custom"
                  />
                  <span>自定义</span>
                  <HelpIcon group="sectionNumbering" option="custom" />
                </label>
              </div>
              
              <!-- 自定义章节配置 -->
              <div v-if="localConfig.sectionNumbering === 'custom'" class="custom-config">
                <div class="custom-config-row">
                  <label>从几级标题开始：</label>
                  <select v-model.number="localConfig.customSectionConfig!.startLevel">
                    <option :value="1">1级 (#)</option>
                    <option :value="2">2级 (##)</option>
                    <option :value="3">3级 (###)</option>
                    <option :value="4">4级 (####)</option>
                    <option :value="5">5级 (#####)</option>
                    <option :value="6">6级 (######)</option>
                  </select>
                </div>
                <div class="custom-config-row">
                  <label>往下几级：</label>
                  <select v-model.number="localConfig.customSectionConfig!.depth">
                    <option :value="1">1级</option>
                    <option :value="2">2级</option>
                    <option :value="3">3级</option>
                    <option :value="4">4级</option>
                    <option :value="5">5级</option>
                    <option :value="6">6级</option>
                  </select>
                </div>
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
              </div>
            </div>
          </div>
        </div>
        
        <div class="dialog-footer">
          <button class="btn-cancel" @click="close">取消</button>
          <button class="btn-confirm" @click="confirm">确定</button>
        </div>
      </div>
    </div>
  </teleport>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import type { TemplateConfig } from '../types/templateConfig'
import HelpIcon from './HelpIcon.vue'

interface Props {
  visible: boolean
  config: Partial<TemplateConfig>
  templatePreset?: Partial<TemplateConfig>
}

interface Emits {
  (e: 'close'): void
  (e: 'confirm', value: Partial<TemplateConfig>): void
  (e: 'load-preset'): void
  (e: 'save-preset'): void
  (e: 'reset'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const localConfig = ref<Partial<TemplateConfig>>({ ...props.config })

// 初始化自定义章节配置
if (!localConfig.value.customSectionConfig) {
  localConfig.value.customSectionConfig = { startLevel: 1, depth: 3 }
}

// 监听章节编号类型变化，自动初始化自定义配置
watch(() => localConfig.value.sectionNumbering, (newValue) => {
  if (newValue === 'custom' && !localConfig.value.customSectionConfig) {
    localConfig.value.customSectionConfig = { startLevel: 1, depth: 3 }
  }
})

// 监听外部配置变化
watch(() => props.config, (newConfig) => {
  localConfig.value = { ...newConfig }
  if (!localConfig.value.date) {
    localConfig.value.date = new Date().toISOString().split('T')[0]
  }
  if (!localConfig.value.customSectionConfig) {
    localConfig.value.customSectionConfig = { startLevel: 1, depth: 3 }
  }
}, { deep: true })

// 监听对话框打开
watch(() => props.visible, (visible) => {
  if (visible) {
    localConfig.value = { ...props.config }
    if (!localConfig.value.date) {
      localConfig.value.date = new Date().toISOString().split('T')[0]
    }
    if (!localConfig.value.customSectionConfig) {
      localConfig.value.customSectionConfig = { startLevel: 1, depth: 3 }
    }
  }
})

function close() {
  emit('close')
}

function confirm() {
  emit('confirm', localConfig.value)
  close()
}

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

<style scoped>
.config-dialog-overlay {
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

.config-dialog {
  background: white;
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  max-width: 800px;
  width: 90%;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  animation: slideUp 0.3s;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes slideUp {
  from {
    transform: translateY(30px);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 24px 28px;
  border-bottom: 1px solid #e5e7eb;
}

.dialog-header h3 {
  margin: 0;
  font-size: 1.5rem;
  color: #1f2937;
  font-weight: 600;
}
.header-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.preset-btn {
  padding: 6px 12px;
  background: #f3f4f6;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 0.85rem;
  font-weight: 500;
  color: #374151;
  cursor: pointer;
  transition: all 0.2s;
}

.preset-btn:hover {
  background: #e5e7eb;
  border-color: #9ca3af;
  color: #1f2937;
}
.close-btn {
  background: none;
  border: none;
  font-size: 2rem;
  line-height: 1;
  color: #9ca3af;
  cursor: pointer;
  padding: 0;
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  transition: all 0.2s;
}

.close-btn:hover {
  background: #f3f4f6;
  color: #374151;
}

.dialog-body {
  padding: 18px 22px;
  overflow-y: auto;
  flex: 1;
}

.config-section {
  margin-bottom: 20px;
  padding-bottom: 20px;
  border-bottom: 1px solid #f3f4f6;
}

.config-section:last-child {
  border-bottom: none;
  margin-bottom: 0;
  padding-bottom: 0;
}

h4 {
  margin: 0 0 12px 0;
  font-size: 1.05rem;
  color: #374151;
  font-weight: 600;
}

.form-group {
  margin-bottom: 12px;
}

.form-group:last-child {
  margin-bottom: 0;
}

label {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 6px;
  font-weight: 500;
  color: #4b5563;
  font-size: 0.9rem;
}

.section-label {
  font-size: 0.95rem;
  color: #374151;
}

input[type="text"],
input[type="date"],
textarea,
select {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  font-size: 0.9rem;
  transition: all 0.2s;
}

input[type="text"]:focus,
input[type="date"]:focus,
textarea:focus,
select:focus {
  outline: none;
  border-color: #4CAF50;
  box-shadow: 0 0 0 3px rgba(76, 175, 80, 0.1);
}

textarea {
  resize: vertical;
  font-family: inherit;
}

select {
  cursor: pointer;
  background: white;
}

.custom-config {
  margin-top: 12px;
  padding: 12px;
  background: #f9fafb;
  border: 1px solid #e5e7eb;
  border-radius: 6px;
}

.custom-config-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 10px;
}

.custom-config-row:last-child {
  margin-bottom: 0;
}

.custom-config-row label {
  min-width: 120px;
  margin-bottom: 0;
  font-size: 0.85rem;
}

.custom-config-row select {
  flex: 1;
  padding: 6px 10px;
  font-size: 0.85rem;
}

.radio-group {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.radio-option {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 7px 12px;
  border: 1.5px solid #e5e7eb;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
  background: white;
  font-size: 0.9rem;
}

.radio-option:hover {
  background: #f9fafb;
  border-color: #9ca3af;
}

.radio-option input[type="radio"] {
  cursor: pointer;
  accent-color: #4CAF50;
}

.radio-option input[type="radio"]:checked + span {
  font-weight: 600;
  color: #16a34a;
}

.radio-option:has(input[type="radio"]:checked) {
  border-color: #4CAF50;
  background: #f0fdf4;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  padding: 16px 22px;
  border-top: 1px solid #e5e7eb;
}

.btn-cancel,
.btn-confirm {
  padding: 9px 24px;
  border: none;
  border-radius: 6px;
  font-size: 0.9rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-cancel {
  background: #f3f4f6;
  color: #6b7280;
}

.btn-cancel:hover {
  background: #e5e7eb;
  color: #374151;
}

.btn-confirm {
  background: #4CAF50;
  color: white;
}

.btn-confirm:hover {
  background: #45a049;
  box-shadow: 0 4px 12px rgba(76, 175, 80, 0.3);
}

.btn-confirm:active {
  transform: translateY(1px);
}
</style>
