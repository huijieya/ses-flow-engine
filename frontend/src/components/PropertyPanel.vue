<template>
  <div class="property-panel">
    <div class="panel-header">
      <h4>属性配置</h4>
      <el-tag v-if="selectedNode" :type="getKindType(selectedNode.data?.kind)" size="small">
        {{ getKindLabel(selectedNode.data?.kind) }}
      </el-tag>
    </div>
    
    <div v-if="selectedNode && nodeDefinition" class="panel-content">
      <el-form label-position="top" size="small">
        <!-- 节点基本信息 -->
        <el-form-item label="节点名称">
          <el-input v-model="form.label" placeholder="请输入节点名称" />
        </el-form-item>
        
        <el-form-item label="节点ID">
          <el-input v-model="form.nodeId" disabled />
        </el-form-item>

        <el-divider />

        <!-- 动态表单 - 根据 config_schema 渲染 -->
        <div v-if="configSchema && configSchema.properties" class="config-section">
          <h5>配置参数</h5>
          <template v-for="(schema, key) in configSchema.properties" :key="key">
            <el-form-item :label="getFieldLabel(key, schema)">
              <!-- 字符串类型 -->
              <el-input
                v-if="schema.type === 'string' && !schema.enum"
                v-model="form.config[key]"
                :placeholder="schema.description || `请输入${key}`"
              />
              
              <!-- 枚举类型 - 下拉选择 -->
              <el-select
                v-else-if="schema.type === 'string' && schema.enum"
                v-model="form.config[key]"
                :placeholder="schema.description || `请选择${key}`"
                style="width: 100%"
              >
                <el-option
                  v-for="option in schema.enum"
                  :key="option"
                  :label="option"
                  :value="option"
                />
              </el-select>
              
              <!-- 数字类型 -->
              <el-input-number
                v-else-if="schema.type === 'number' || schema.type === 'integer'"
                v-model="form.config[key]"
                :placeholder="schema.description"
                style="width: 100%"
              />
              
              <!-- 布尔类型 -->
              <el-switch
                v-else-if="schema.type === 'boolean'"
                v-model="form.config[key]"
                :active-text="form.config[key] ? '是' : '否'"
              />
              
              <!-- 数组类型 -->
              <div v-else-if="schema.type === 'array'" class="array-input">
                <div v-for="(item, index) in (form.config[key] || [])" :key="index" class="array-item">
                  <el-input v-model="form.config[key][index]" size="small" />
                  <el-button type="danger" link size="small" @click="removeArrayItem(key, index)">
                    <el-icon><Delete /></el-icon>
                  </el-button>
                </div>
                <el-button type="primary" link size="small" @click="addArrayItem(key)">
                  <el-icon><Plus /></el-icon>添加项
                </el-button>
              </div>
              
              <!-- 对象类型 -->
              <div v-else-if="schema.type === 'object'" class="object-input">
                <el-input
                  v-model="form.config[key]"
                  type="textarea"
                  :rows="3"
                  :placeholder="`JSON对象格式，例如: ${getExampleObject(schema)}`"
                />
              </div>
              
              <!-- 其他类型 -->
              <el-input
                v-else
                v-model="form.config[key]"
                :placeholder="schema.description || `请输入${key}`"
              />
            </el-form-item>
          </template>
        </div>

        <!-- 无配置项提示 -->
        <el-empty v-else description="该节点无配置参数" :image-size="60" />
      </el-form>
    </div>

    <div v-else class="empty-state">
      <el-icon :size="48" color="#dcdfe6"><InfoFilled /></el-icon>
      <p>选择节点以编辑属性</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { Delete, Plus } from '@element-plus/icons-vue'

interface ConfigSchema {
  type?: string
  properties?: Record<string, any>
  description?: string
  enum?: string[]
  default?: any
}

interface NodeDefinition {
  node_id: string
  name: string
  description?: string
  kind: string
  config_schema?: ConfigSchema
  default_config?: Record<string, any>
}

const props = defineProps<{
  selectedNode: any
  nodeDefinition?: NodeDefinition
}>()

const emit = defineEmits(['update:config'])

const form = ref({
  label: '',
  nodeId: '',
  config: {} as Record<string, any>
})

// 计算配置 schema
const configSchema = computed(() => {
  return props.nodeDefinition?.config_schema || null
})

// 获取字段标签
const getFieldLabel = (key: string, schema: any): string => {
  return schema.title || schema.description || key
}

// 获取示例对象
const getExampleObject = (schema: any): string => {
  if (schema.properties) {
    const example: Record<string, any> = {}
    Object.entries(schema.properties).forEach(([k, v]: [string, any]) => {
      if (v.type === 'boolean') example[k] = true
      else if (v.type === 'number' || v.type === 'integer') example[k] = 0
      else example[k] = 'value'
    })
    return JSON.stringify(example)
  }
  return '{}'
}

// 数组操作
const addArrayItem = (key: string) => {
  if (!form.value.config[key]) {
    form.value.config[key] = []
  }
  form.value.config[key].push('')
}

const removeArrayItem = (key: string, index: number) => {
  if (form.value.config[key]) {
    form.value.config[key].splice(index, 1)
  }
}

// 获取节点类型标签
const getKindLabel = (kind?: string): string => {
  const labels: Record<string, string> = {
    device: '设备节点',
    logic: '逻辑节点',
    data: '数据节点',
    system: '系统节点',
    query: '查询节点'
  }
  return labels[kind || ''] || kind || '未知'
}

// 获取节点类型样式
const getKindType = (kind?: string): any => {
  const types: Record<string, any> = {
    device: 'primary',
    logic: 'success',
    data: 'warning',
    system: 'info',
    query: 'danger'
  }
  return types[kind || ''] || 'info'
}

// 深度合并默认值
const mergeWithDefaults = (schema: ConfigSchema, existingConfig: Record<string, any>): Record<string, any> => {
  const result: Record<string, any> = {}
  
  if (schema.properties) {
    Object.entries(schema.properties).forEach(([key, propSchema]: [string, any]) => {
      // 优先使用现有值，其次使用 schema 默认值，最后根据类型设置默认值
      if (existingConfig[key] !== undefined) {
        result[key] = existingConfig[key]
      } else if (propSchema.default !== undefined) {
        result[key] = propSchema.default
      } else {
        // 根据类型设置默认空值
        switch (propSchema.type) {
          case 'string':
            result[key] = ''
            break
          case 'number':
          case 'integer':
            result[key] = 0
            break
          case 'boolean':
            result[key] = false
            break
          case 'array':
            result[key] = []
            break
          case 'object':
            result[key] = {}
            break
          default:
            result[key] = null
        }
      }
    })
  }
  
  return result
}

// 监听选中节点变化
watch(() => props.selectedNode, (node) => {
  if (node) {
    const existingConfig = node.data?.config || {}
    const schema = props.nodeDefinition?.config_schema
    
    form.value = {
      label: node.data?.label || node.label || '',
      nodeId: node.id || '',
      config: schema ? mergeWithDefaults(schema, existingConfig) : { ...existingConfig }
    }
  }
}, { immediate: true, deep: true })

// 监听表单变化，同步回父组件
watch(() => form.value, (newForm) => {
  if (props.selectedNode) {
    emit('update:config', {
      nodeId: props.selectedNode.id,
      data: {
        label: newForm.label,
        config: newForm.config
      }
    })
  }
}, { deep: true })
</script>

<style scoped>
.property-panel {
  width: 320px;
  background: #fff;
  border-left: 1px solid #dcdfe6;
  display: flex;
  flex-direction: column;
}

.panel-header {
  padding: 16px;
  border-bottom: 1px solid #ebeef5;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.panel-header h4 {
  margin: 0;
  font-size: 16px;
  color: #303133;
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

.config-section h5 {
  margin: 0 0 12px 0;
  font-size: 14px;
  color: #606266;
}

.array-input {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.array-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.object-input :deep(.el-textarea__inner) {
  font-family: monospace;
  font-size: 12px;
}
</style>
