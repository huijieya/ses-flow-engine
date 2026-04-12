<template>
  <div class="property-panel">
    <div class="panel-header">
      <h4>属性配置</h4>
    </div>
    
    <div v-if="selectedNode" class="panel-content">
      <el-form label-position="top">
        <el-form-item label="节点名称">
          <el-input v-model="form.label" />
        </el-form-item>
        
        <el-form-item label="节点类型">
          <el-input v-model="form.type" disabled />
        </el-form-item>

        <el-divider />

        <h5>配置参数</h5>
        
        <div v-for="(value, key) in form.config" :key="key" class="config-row">
          <el-form-item :label="key">
            <el-input v-model="form.config[key]" />
          </el-form-item>
        </div>

        <el-button type="primary" link @click="showAddConfig = true">
          <el-icon><Plus /></el-icon>添加参数
        </el-button>
      </el-form>
    </div>

    <div v-else class="empty-state">
      <el-icon :size="48" color="#dcdfe6"><InfoFilled /></el-icon>
      <p>选择节点以编辑属性</p>
    </div>

    <el-dialog v-model="showAddConfig" title="添加参数" width="400px">
      <el-form label-width="80px">
        <el-form-item label="参数名">
          <el-input v-model="newConfigKey" placeholder="请输入参数名" />
        </el-form-item>
        <el-form-item label="参数值">
          <el-input v-model="newConfigValue" placeholder="请输入参数值" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAddConfig = false">取消</el-button>
        <el-button type="primary" @click="addConfig">确定</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{
  selectedNode: any
}>()

const form = ref({
  label: '',
  type: '',
  config: {} as Record<string, string>
})

const showAddConfig = ref(false)
const newConfigKey = ref('')
const newConfigValue = ref('')

watch(() => props.selectedNode, (node) => {
  if (node) {
    form.value = {
      label: node.data?.label || '',
      type: node.data?.type || '',
      config: { ...(node.data?.config || {}) }
    }
  }
}, { immediate: true })

const addConfig = () => {
  if (newConfigKey.value) {
    form.value.config[newConfigKey.value] = newConfigValue.value
    newConfigKey.value = ''
    newConfigValue.value = ''
    showAddConfig.value = false
  }
}
</script>

<style scoped>
.property-panel {
  width: 300px;
  background: #fff;
  border-left: 1px solid #dcdfe6;
  display: flex;
  flex-direction: column;
}

.panel-header {
  padding: 16px;
  border-bottom: 1px solid #ebeef5;
}

.panel-header h4 {
  margin: 0;
}

.panel-content {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #909399;
}

.config-row {
  margin-bottom: 8px;
}
</style>
