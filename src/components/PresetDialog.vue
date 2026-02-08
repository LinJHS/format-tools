<template>
  <teleport to="body">
    <div v-if="visible" class="preset-dialog-overlay" @click="close">
      <div class="preset-dialog" @click.stop>
        <div class="dialog-header">
          <h3>{{ mode === 'load' ? '加载预设' : '保存预设' }}</h3>
          <button class="close-btn" @click="close">&times;</button>
        </div>
        
        <div class="dialog-body">
          <!-- 加载预设模式 -->
          <div v-if="mode === 'load'">
            <div class="preset-tabs">
              <button 
                :class="['tab-btn', { active: activeTab === 'builtin' }]"
                @click="activeTab = 'builtin'"
              >
                内置预设
              </button>
              <button 
                :class="['tab-btn', { active: activeTab === 'custom' }]"
                @click="activeTab = 'custom'"
              >
                自定义预设
              </button>
              <button 
                :class="['tab-btn', { active: activeTab === 'recent' }]"
                @click="activeTab = 'recent'"
              >
                历史记录
              </button>
            </div>
            
            <!-- 内置预设列表 -->
            <div v-if="activeTab === 'builtin'" class="preset-list">
              <div 
                v-for="preset in builtinPresets" 
                :key="preset.id"
                class="preset-item"
                @click="selectPreset(preset)"
              >
                <div class="preset-info">
                  <h4>{{ preset.name }}</h4>
                  <p>{{ preset.description }}</p>
                </div>
                <div class="preset-badge builtin">内置</div>
              </div>
            </div>
            
            <!-- 自定义预设列表 -->
            <div v-if="activeTab === 'custom'" class="preset-list">
              <div v-if="customPresets.length === 0" class="empty-state">
                <p>暂无自定义预设</p>
                <p class="hint">保存当前配置即可创建自定义预设</p>
              </div>
              <div 
                v-for="preset in customPresets" 
                :key="preset.id"
                class="preset-item"
              >
                <div class="preset-info" @click="selectPreset(preset)">
                  <h4>{{ preset.name }}</h4>
                  <p>{{ preset.description || '无描述' }}</p>
                </div>
                <button class="delete-btn" @click.stop="deletePreset(preset.id)">
                  删除
                </button>
              </div>
            </div>
            
            <!-- 历史记录列表 -->
            <div v-if="activeTab === 'recent'" class="preset-list">
              <div v-if="recentConfigs.length === 0" class="empty-state">
                <p>暂无历史记录</p>
              </div>
              <div 
                v-for="(item, index) in recentConfigs" 
                :key="index"
                class="preset-item"
                @click="loadRecentConfig(item.config)"
              >
                <div class="preset-info">
                  <h4>历史配置 {{ index + 1 }}</h4>
                  <p class="timestamp">{{ formatTimestamp(item.timestamp) }}</p>
                </div>
              </div>
            </div>
          </div>
          
          <!-- 保存预设模式 -->
          <div v-if="mode === 'save'" class="save-form">
            <div class="form-group">
              <label>预设名称 *</label>
              <input 
                v-model="presetName" 
                type="text" 
                placeholder="输入预设名称"
                maxlength="50"
              />
            </div>
            <div class="form-group">
              <label>描述</label>
              <textarea 
                v-model="presetDescription" 
                placeholder="可选，描述此预设的用途"
                rows="3"
                maxlength="200"
              ></textarea>
            </div>
            <div v-if="saveError" class="error-message">{{ saveError }}</div>
          </div>
        </div>
        
        <div class="dialog-footer">
          <button class="btn-secondary" @click="close">取消</button>
          <button 
            v-if="mode === 'save'" 
            class="btn-primary" 
            :disabled="!presetName.trim()"
            @click="savePreset"
          >
            保存
          </button>
        </div>
      </div>
    </div>
  </teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { BUILTIN_PRESETS } from '../types/templateConfig'
import type { ConfigPreset, TemplateConfig } from '../types/templateConfig'
import { 
  getCustomPresets, 
  saveCustomPreset, 
  deleteCustomPreset,
  getRecentConfigs 
} from '../services/configStorage'

interface Props {
  visible: boolean
  mode: 'load' | 'save'
  currentConfig?: Partial<TemplateConfig>
}

interface Emits {
  (e: 'close'): void
  (e: 'load', config: Partial<TemplateConfig>): void
  (e: 'save', preset: ConfigPreset): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const activeTab = ref<'builtin' | 'custom' | 'recent'>('builtin')
const presetName = ref('')
const presetDescription = ref('')
const saveError = ref('')

const customPresets = ref<ConfigPreset[]>([])
const recentConfigs = ref<Array<{ config: Partial<TemplateConfig>, timestamp: string }>>([])

const builtinPresets = computed(() => BUILTIN_PRESETS)

// 监听对话框打开，刷新数据
watch(() => props.visible, (visible) => {
  if (visible) {
    if (props.mode === 'load') {
      refreshPresets()
    } else {
      presetName.value = ''
      presetDescription.value = ''
      saveError.value = ''
    }
  }
})

function refreshPresets() {
  customPresets.value = getCustomPresets()
  recentConfigs.value = getRecentConfigs()
}

function close() {
  emit('close')
}

function selectPreset(preset: ConfigPreset) {
  emit('load', preset.config)
  close()
}

function loadRecentConfig(config: Partial<TemplateConfig>) {
  emit('load', config)
  close()
}

function savePreset() {
  if (!presetName.value.trim()) {
    saveError.value = '请输入预设名称'
    return
  }
  
  if (!props.currentConfig) {
    saveError.value = '当前配置为空'
    return
  }
  
  try {
    const preset = saveCustomPreset({
      name: presetName.value.trim(),
      description: presetDescription.value.trim() || undefined,
      config: props.currentConfig
    })
    
    emit('save', preset)
    close()
  } catch (error) {
    saveError.value = `保存失败: ${error}`
  }
}

function deletePreset(id: string) {
  if (confirm('确定要删除此预设吗？')) {
    deleteCustomPreset(id)
    refreshPresets()
  }
}

function formatTimestamp(timestamp: string): string {
  const date = new Date(timestamp)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  
  // 1小时内
  if (diff < 3600000) {
    const minutes = Math.floor(diff / 60000)
    return `${minutes} 分钟前`
  }
  
  // 24小时内
  if (diff < 86400000) {
    const hours = Math.floor(diff / 3600000)
    return `${hours} 小时前`
  }
  
  // 显示日期
  return date.toLocaleDateString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}
</script>

<style scoped>
.preset-dialog-overlay {
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

.preset-dialog {
  background: white;
  border-radius: 12px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
  max-width: 700px;
  width: 90%;
  max-height: 80vh;
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
    transform: translateY(20px);
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
  padding: 24px;
  border-bottom: 1px solid #eee;
}

.dialog-header h3 {
  margin: 0;
  font-size: 1.4rem;
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
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  transition: all 0.2s;
}

.close-btn:hover {
  background: #f5f5f5;
  color: #333;
}

.dialog-body {
  padding: 24px;
  overflow-y: auto;
  flex: 1;
}

.preset-tabs {
  display: flex;
  gap: 8px;
  margin-bottom: 20px;
  border-bottom: 2px solid #eee;
}

.tab-btn {
  padding: 10px 20px;
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  color: #666;
  cursor: pointer;
  font-size: 0.95rem;
  font-weight: 500;
  transition: all 0.2s;
  margin-bottom: -2px;
}

.tab-btn:hover {
  color: #333;
}

.tab-btn.active {
  color: #4CAF50;
  border-bottom-color: #4CAF50;
}

.preset-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.preset-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  border: 1px solid #ddd;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.preset-item:hover {
  border-color: #4CAF50;
  background: #f9fdf9;
}

.preset-info {
  flex: 1;
}

.preset-info h4 {
  margin: 0 0 6px 0;
  font-size: 1.05rem;
  color: #333;
}

.preset-info p {
  margin: 0;
  font-size: 0.9rem;
  color: #666;
}

.timestamp {
  font-size: 0.85rem;
  color: #999;
}

.preset-badge {
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 0.8rem;
  font-weight: 600;
}

.preset-badge.builtin {
  background: #e3f2fd;
  color: #1976d2;
}

.delete-btn {
  padding: 6px 14px;
  background: #ffebee;
  color: #c62828;
  border: 1px solid #ffcdd2;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.85rem;
  transition: all 0.2s;
}

.delete-btn:hover {
  background: #ffcdd2;
  border-color: #ef5350;
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
  color: #999;
}

.empty-state p {
  margin: 0 0 8px 0;
  font-size: 0.95rem;
}

.empty-state .hint {
  font-size: 0.85rem;
  color: #bbb;
}

.save-form {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-group label {
  font-weight: 500;
  color: #333;
}

.form-group input,
.form-group textarea {
  padding: 10px 14px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 0.95rem;
  transition: border-color 0.2s;
}

.form-group input:focus,
.form-group textarea:focus {
  outline: none;
  border-color: #4CAF50;
}

.form-group textarea {
  resize: vertical;
  font-family: inherit;
}

.error-message {
  padding: 10px;
  background: #ffebee;
  color: #c62828;
  border-radius: 6px;
  font-size: 0.9rem;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 20px 24px;
  border-top: 1px solid #eee;
}

.btn-secondary,
.btn-primary {
  padding: 10px 24px;
  border: none;
  border-radius: 6px;
  font-size: 0.95rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary {
  background: #f5f5f5;
  color: #666;
}

.btn-secondary:hover {
  background: #e9e9e9;
}

.btn-primary {
  background: #4CAF50;
  color: white;
}

.btn-primary:hover {
  background: #45a049;
}

.btn-primary:disabled {
  background: #ccc;
  cursor: not-allowed;
}
</style>
