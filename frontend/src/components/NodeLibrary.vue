<template>
  <div class="node-library">
    <div class="library-header">
      <h4>节点库</h4>
      <el-input v-model="searchText" placeholder="搜索节点" size="small" clearable>
        <template #prefix>
          <el-icon><Search /></el-icon>
        </template>
      </el-input>
    </div>

    <div class="library-content">
      <el-collapse v-model="activeCategories">
        <el-collapse-item 
          v-for="category in filteredCategories" 
          :key="category.name" 
          :title="`${category.name} (${category.nodes.length})`" 
          :name="category.name"
        >
          <div class="node-list">
            <div
              v-for="node in category.nodes"
              :key="node.node_id"
              class="node-item"
              :class="[`node-${node.kind}`]"
              draggable="true"
              @dragstart="(e) => onDragStart(e, node)"
            >
              <div class="node-icon" :style="{ backgroundColor: node.color || '#909399' }">
                <el-icon :size="16" color="#fff">
                  <component :is="getIcon(node.icon)" />
                </el-icon>
              </div>
              <div class="node-info">
                <span class="node-label">{{ node.name }}</span>
                <span class="node-desc" v-if="node.description">{{ node.description }}</span>
              </div>
            </div>
          </div>
        </el-collapse-item>
      </el-collapse>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { getNodeDefinitions } from '@/api/nodes'
import { ElMessage } from 'element-plus'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'

interface NodeDefinition {
  id: string
  node_id: string
  name: string
  description?: string
  kind: 'device' | 'logic' | 'data' | 'system'
  device_type?: string
  input_schema?: any
  output_schema?: any
  config_schema?: any
  default_config?: any
  icon?: string
  color?: string
  category: string
  is_system: boolean
}

const searchText = ref('')
const activeCategories = ref<string[]>([])
const nodeDefinitions = ref<NodeDefinition[]>([])
const loading = ref(false)

// 从后端加载节点定义
const loadNodeDefinitions = async () => {
  loading.value = true
  try {
    const result = await getNodeDefinitions() as any
    if (result.data) {
      nodeDefinitions.value = result.data
      // 默认展开所有分类
      const categories = [...new Set(result.data.map((n: NodeDefinition) => n.category))]
      activeCategories.value = categories
    }
  } catch (error) {
    console.error('加载节点定义失败:', error)
    ElMessage.error('加载节点定义失败')
  } finally {
    loading.value = false
  }
}

// 按分类分组
const categories = computed(() => {
  const grouped = nodeDefinitions.value.reduce((acc, node) => {
    const category = node.category || '其他'
    if (!acc[category]) {
      acc[category] = []
    }
    acc[category].push(node)
    return acc
  }, {} as Record<string, NodeDefinition[]>)

  return Object.entries(grouped).map(([name, nodes]) => ({
    name,
    nodes: nodes.sort((a, b) => a.name.localeCompare(b.name))
  }))
})

// 搜索过滤
const filteredCategories = computed(() => {
  if (!searchText.value) return categories.value
  const search = searchText.value.toLowerCase()
  return categories.value.map(cat => ({
    ...cat,
    nodes: cat.nodes.filter(node => 
      node.name.toLowerCase().includes(search) ||
      node.node_id.toLowerCase().includes(search) ||
      (node.description && node.description.toLowerCase().includes(search))
    )
  })).filter(cat => cat.nodes.length > 0)
})

// 获取图标组件
const getIcon = (iconName?: string) => {
  if (!iconName) return 'CircleCheck'
  return (ElementPlusIconsVue as any)[iconName] || 'CircleCheck'
}

// 拖拽开始
const onDragStart = (event: DragEvent, node: NodeDefinition) => {
  if (event.dataTransfer) {
    event.dataTransfer.setData('application/vueflow', JSON.stringify(node))
    event.dataTransfer.effectAllowed = 'move'
  }
}

onMounted(() => {
  loadNodeDefinitions()
})
</script>

<style scoped>
.node-library {
  width: 280px;
  background: #fff;
  border-right: 1px solid #dcdfe6;
  display: flex;
  flex-direction: column;
}

.library-header {
  padding: 16px;
  border-bottom: 1px solid #ebeef5;
}

.library-header h4 {
  margin: 0 0 12px 0;
  font-size: 16px;
  color: #303133;
}

.library-content {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.node-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.node-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border-radius: 6px;
  cursor: move;
  transition: all 0.2s;
  border: 1px solid #ebeef5;
  background: #fff;
}

.node-item:hover {
  background: #f5f7fa;
  border-color: #2EC6D6;
  box-shadow: 0 2px 8px rgba(46, 198, 214, 0.15);
}

.node-icon {
  width: 32px;
  height: 32px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.node-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  flex: 1;
  min-width: 0;
}

.node-label {
  font-size: 13px;
  color: #303133;
  font-weight: 500;
}

.node-desc {
  font-size: 11px;
  color: #909399;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.node-device {
  border-left: 3px solid #409EFF;
}

.node-logic {
  border-left: 3px solid #67C23A;
}

.node-data {
  border-left: 3px solid #E6A23C;
}

.node-system {
  border-left: 3px solid #909399;
}
</style>
