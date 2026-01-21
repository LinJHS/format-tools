<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useUploadStore } from '../stores/upload'
import { useRouter } from 'vue-router'
import { pandocService, TemplateInfo } from '../services/pandocService'

const uploadStore = useUploadStore()
const router = useRouter()

interface Template {
  id: string
  name: string
  description: string
  icon: string
  isFree: boolean
  isPro: boolean
}

const templates = ref<Template[]>([
  {
    id: 'free',
    name: '经典模板',
    description: '简洁专业的 Word 文档模板，适合大多数场景使用',
    icon: '📄',
    isFree: true,
    isPro: true
  }
])

const selectedTemplate = ref<Template | null>(templates.value[0])
const isLoading = ref(false)
const error = ref('')

onMounted(() => {
  if (!uploadStore.preparedInput) {
    router.push('/upload')
  }
  selectedTemplate.value = templates.value[0]
})

const selectTemplate = (template: Template) => {
  selectedTemplate.value = template
  error.value = ''
}

const convertMarkdown = async () => {
  if (!selectedTemplate.value) return

  try {
    isLoading.value = true
    error.value = ''

    // Prepare template
    const templateInfo: TemplateInfo = await pandocService.prepareTemplate(selectedTemplate.value.id)

    // Prepare convert options
    const preparedInput = uploadStore.preparedInput
    if (!preparedInput) {
      throw new Error('Input not prepared')
    }

    const convertOptions = {
      input_file: preparedInput.markdown_path,
      output_file: preparedInput.markdown_path.replace(/\.md$|\.txt$/, '.docx'),
      reference_doc: templateInfo.reference_doc,
      metadata_file: undefined,
      use_crossref: true
    }

    // Convert markdown
    const outputPath = await pandocService.convertMarkdown(convertOptions)

    // Show success message and navigate
    console.log('转换完成:', outputPath)
    uploadStore.setStep(3)
    router.push('/result')
  } catch (err) {
    console.error('转换失败:', err)
    error.value = `转换失败: ${err instanceof Error ? err.message : String(err)}`
  } finally {
    isLoading.value = false
  }
}

const goBack = () => {
  router.push('/upload')
}

const templatesByCategory = computed(() => {
  return templates.value
})
</script>

<template>
  <div class="template-container">
    <div class="header">
      <div>
        <h1>选择模板</h1>
        <p class="subtitle">选择一个适合你的文档模板，然后开始转换</p>
      </div>
      <span class="badge">Step 2 / 2</span>
    </div>

    <!-- Free Templates Section -->
    <section class="templates-section">
      <h2 class="section-title">免费模板</h2>
      <div class="templates-grid">
        <div
          v-for="template in templatesByCategory"
          :key="template.id"
          class="template-card"
          :class="{ selected: selectedTemplate?.id === template.id }"
          @click="selectTemplate(template)"
        >
          <div class="card-header">
            <div class="icon">{{ template.icon }}</div>
            <div v-if="template.isFree" class="badge-free">免费</div>
          </div>

          <div class="card-body">
            <h3>{{ template.name }}</h3>
            <p>{{ template.description }}</p>
          </div>

          <div class="card-footer">
            <div
              v-if="selectedTemplate?.id === template.id"
              class="checkmark"
            >
              ✓ 已选择
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Pro Templates Section (Placeholder) -->
    <section class="templates-section pro-section">
      <h2 class="section-title">高级模板</h2>
      <div class="coming-soon">
        <div class="icon">🚀</div>
        <p>敬请期待更多专业模板</p>
        <p class="sub-text">登录后可解锁高级模板</p>
      </div>
    </section>

    <!-- Error Message -->
    <div v-if="error" class="error-box">{{ error }}</div>

    <!-- Action Buttons -->
    <div class="button-group">
      <button class="btn-back" @click="goBack">上一步</button>
      <button
        class="btn-convert"
        :disabled="!selectedTemplate || isLoading"
        @click="convertMarkdown"
      >
        {{ isLoading ? '转换中...' : '开始转换' }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.template-container {
  min-height: 100vh;
  padding: 32px;
  background: radial-gradient(circle at 20% 20%, #f5f7ff, #eef2ff 40%, #e8edf8 80%);
}

.header {
  max-width: 1200px;
  margin: 0 auto 28px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

h1 {
  margin: 0;
  color: #1f2937;
  font-size: 28px;
  letter-spacing: -0.4px;
}

.subtitle {
  margin: 4px 0 0 0;
  color: #4b5563;
  font-size: 14px;
}

.badge {
  background: #e0e7ff;
  color: #3730a3;
  padding: 8px 12px;
  border-radius: 12px;
  font-weight: 600;
  font-size: 12px;
  white-space: nowrap;
}

.templates-section {
  max-width: 1200px;
  margin: 0 auto 32px;
}

.section-title {
  font-size: 18px;
  font-weight: 700;
  color: #1f2937;
  margin: 0 0 16px 0;
}

.templates-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}

.template-card {
  background: white;
  border: 2px solid #e5e7eb;
  border-radius: 16px;
  padding: 20px;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  flex-direction: column;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
}

.template-card:hover {
  border-color: #c7d2fe;
  box-shadow: 0 4px 16px rgba(99, 102, 241, 0.15);
  transform: translateY(-2px);
}

.template-card.selected {
  border-color: #7c3aed;
  background: #faf5ff;
  box-shadow: 0 4px 20px rgba(124, 58, 237, 0.2);
}

.card-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: 12px;
}

.icon {
  font-size: 32px;
}

.badge-free {
  background: #dcfce7;
  color: #166534;
  padding: 4px 8px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 600;
}

.card-body {
  flex: 1;
}

.card-body h3 {
  margin: 0 0 8px 0;
  color: #111827;
  font-size: 18px;
  font-weight: 700;
}

.card-body p {
  margin: 0;
  color: #6b7280;
  font-size: 13px;
  line-height: 1.5;
}

.card-footer {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid #e5e7eb;
}

.checkmark {
  color: #16a34a;
  font-weight: 700;
  font-size: 14px;
}

.pro-section {
  opacity: 0.6;
}

.coming-soon {
  background: linear-gradient(135deg, #f5f7ff, #ede9fe);
  border: 2px dashed #c7d2fe;
  border-radius: 16px;
  padding: 40px;
  text-align: center;
  color: #7c3aed;
}

.coming-soon .icon {
  font-size: 48px;
  display: block;
  margin-bottom: 12px;
}

.coming-soon p {
  margin: 0;
  font-weight: 600;
}

.coming-soon .sub-text {
  color: #a78bfa;
  font-size: 12px;
  margin-top: 4px;
}

.error-box {
  max-width: 1200px;
  margin: 0 auto 16px;
  padding: 12px 14px;
  border-radius: 12px;
  background: #fef2f2;
  color: #b91c1c;
  border: 1px solid #fecdd3;
}

.button-group {
  max-width: 1200px;
  margin: 32px auto 0;
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.btn-back {
  background: #e5e7eb;
  color: #374151;
  padding: 12px 24px;
  border: none;
  border-radius: 12px;
  font-size: 16px;
  font-weight: 700;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-back:hover {
  background: #d1d5db;
}

.btn-convert {
  background: linear-gradient(90deg, #22c55e, #16a34a);
  color: #fff;
  padding: 12px 28px;
  border: none;
  border-radius: 12px;
  font-size: 16px;
  font-weight: 700;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 12px 30px rgba(34, 197, 94, 0.25);
}

.btn-convert:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 14px 34px rgba(34, 197, 94, 0.3);
}

.btn-convert:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  box-shadow: none;
}

@media (max-width: 768px) {
  .template-container {
    padding: 18px;
  }

  .header {
    flex-direction: column;
    align-items: flex-start;
  }

  h1 {
    font-size: 22px;
  }

  .templates-grid {
    grid-template-columns: 1fr;
  }

  .button-group {
    flex-direction: column-reverse;
  }

  .btn-back,
  .btn-convert {
    width: 100%;
  }
}
</style>
