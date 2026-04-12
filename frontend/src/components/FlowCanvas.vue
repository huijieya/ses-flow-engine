<template>
  <div class="flow-canvas">
    <VueFlow
      v-model="elements"
      :default-zoom="1"
      :min-zoom="0.2"
      :max-zoom="4"
      @dragover="onDragOver"
      @drop="onDrop"
      @connect="onConnect"
      @node-click="onNodeClick"
      @pane-click="onPaneClick"
    >
      <Background pattern-color="#aaa" :gap="16" />
      <MiniMap />
      <Controls />
      
      <template #node-custom="{ data, id }">
        <div 
          class="custom-node" 
          :class="[`node-${data.kind}`, { 'node-selected': selectedNodeId === id }]"
          @click.stop="selectNode(id)"
        >
          <div class="node-header" :style="{ borderLeftColor: data.color || '#909399' }">
            <div class="node-icon" :style="{ backgroundColor: data.color || '#909399' }">
              <el-icon :size="14" color="#fff">
                <component :is="getIcon(data.icon)" />
              </el-icon>
            </div>
            <span class="node-title">{{ data.label }}</span>
          </div>
          <div class="node-body">
            <div v-if="hasConfig(data.config)" class="node-config-preview">
              <div v-for="(value, key) in getConfigPreview(data.config)" :key="key" class="config-preview-item">
                <span class="config-key">{{ key }}:</span>
                <span class="config-value">{{ formatValue(value) }}</span>
              </div>
            </div>
            <div v-else class="node-no-config">
              点击配置参数
            </div>
          </div>
          <Handle type="target" :position="Position.Top" />
          <Handle type="source" :position="Position.Bottom" />
        </div>
      </template>
    </VueFlow>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { VueFlow, useVueFlow, Handle, Position } from '@vue-flow/core'
import { Background } from '@vue-flow/background'
import { MiniMap } from '@vue-flow/minimap'
import { Controls } from '@vue-flow/controls'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import '@vue-flow/core/dist/style.css'
import '@vue-flow/controls/dist/style.css'

interface NodeData {
  label: string
  kind: string
  icon?: string
  color?: string
  nodeId?: string
  config?: Record<string, any>
}

const props = defineProps<{
  nodes: any[]
  edges: any[]
}>()

const emit = defineEmits(['update:nodes', 'update:edges', 'node-select', 'node-deselect'])

const { addNodes, addEdges, project, findNode } = useVueFlow()
const selectedNodeId = ref<string | null>(null)

const elements = computed({
  get: () => [...props.nodes, ...props.edges],
  set: (val) => {
    const n = val.filter((el: any) => !el.source)
    const e = val.filter((el: any) => el.source)
    emit('update:nodes', n)
    emit('update:edges', e)
  }
})

// 获取图标组件
const getIcon = (iconName?: string) => {
  if (!iconName) return 'CircleCheck'
  return (ElementPlusIconsVue as any)[iconName] || 'CircleCheck'
}

// 检查是否有配置
const hasConfig = (config?: Record<string, any>): boolean => {
  if (!config) return false
  return Object.keys(config).length > 0 && Object.values(config).some(v => v !== '' && v !== null && v !== undefined)
}

// 获取配置预览（最多显示3个）
const getConfigPreview = (config?: Record<string, any>): Record<string, any> => {
  if (!config) return {}
  const entries = Object.entries(config).filter(([_, v]) => v !== '' && v !== null && v !== undefined)
  return Object.fromEntries(entries.slice(0, 3))
}

// 格式化值显示
const formatValue = (value: any): string => {
  if (value === null || value === undefined) return '-'
  if (typeof value === 'boolean') return value ? '是' : '否'
  if (typeof value === 'object') return JSON.stringify(value).slice(0, 20)
  return String(value).slice(0, 20)
}

const onDragOver = (event: DragEvent) => {
  event.preventDefault()
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'move'
  }
}

const onDrop = (event: DragEvent) => {
  event.preventDefault()
  const data = event.dataTransfer?.getData('application/vueflow')
  if (!data) return

  try {
    const nodeDef = JSON.parse(data)
    const position = project({ x: event.clientX - 300, y: event.clientY - 100 })

    const newNode = {
      id: `node_${Date.now()}`,
      type: 'custom',
      position,
      data: {
        label: nodeDef.name,
        kind: nodeDef.kind,
        icon: nodeDef.icon,
        color: nodeDef.color,
        nodeId: nodeDef.node_id,
        config: nodeDef.default_config || {}
      },
    }

    addNodes([newNode])
  } catch (error) {
    console.error('创建节点失败:', error)
  }
}

const onConnect = (connection: any) => {
  addEdges([{
    id: `edge_${Date.now()}`,
    source: connection.source,
    target: connection.target,
    animated: true,
  }])
}

const selectNode = (id: string) => {
  selectedNodeId.value = id
  const node = findNode(id)
  if (node) {
    emit('node-select', node)
  }
}

const onNodeClick = (event: any, node: any) => {
  selectedNodeId.value = node.id
  emit('node-select', node)
}

const onPaneClick = () => {
  selectedNodeId.value = null
  emit('node-deselect')
}

// 更新节点数据的方法（供父组件调用）
const updateNodeData = (nodeId: string, newData: Partial<NodeData>) => {
  const node = findNode(nodeId)
  if (node) {
    node.data = { ...node.data, ...newData }
  }
}

// 暴露方法给父组件
defineExpose({
  updateNodeData
})
</script>

<style scoped>
.flow-canvas {
  flex: 1;
  height: 100%;
}

.custom-node {
  background: #fff;
  border: 2px solid #dcdfe6;
  border-radius: 8px;
  min-width: 160px;
  max-width: 220px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  transition: all 0.2s;
  cursor: pointer;
}

.custom-node:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.12);
}

.node-selected {
  border-color: #2EC6D6;
  box-shadow: 0 0 0 3px rgba(46, 198, 214, 0.2);
}

.node-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 10px;
  background: #f5f7fa;
  border-radius: 6px 6px 0 0;
  border-left: 3px solid;
}

.node-icon {
  width: 24px;
  height: 24px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.node-title {
  font-weight: 500;
  font-size: 13px;
  color: #303133;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.node-body {
  padding: 8px 10px;
  min-height: 30px;
}

.node-config-preview {
  font-size: 11px;
}

.config-preview-item {
  display: flex;
  gap: 4px;
  margin-bottom: 2px;
  white-space: nowrap;
  overflow: hidden;
}

.config-key {
  color: #606266;
  font-weight: 500;
}

.config-value {
  color: #2EC6D6;
  overflow: hidden;
  text-overflow: ellipsis;
}

.node-no-config {
  font-size: 11px;
  color: #c0c4cc;
  font-style: italic;
}

/* 节点类型样式 */
.node-device {
  border-left-color: #409EFF;
}

.node-logic {
  border-left-color: #67C23A;
}

.node-data {
  border-left-color: #E6A23C;
}

.node-system {
  border-left-color: #909399;
}
</style>
