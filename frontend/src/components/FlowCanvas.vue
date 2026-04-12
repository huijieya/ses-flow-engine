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
    >
      <Background pattern-color="#aaa" :gap="16" />
      <MiniMap />
      <Controls />
      
      <template #node-custom="{ data, id }">
        <div class="custom-node" :class="`node-${data.kind}`">
          <div class="node-header">
            <el-icon><component :is="data.icon || 'CircleCheck'" /></el-icon>
            <span>{{ data.label }}</span>
          </div>
          <div class="node-body">
            <div v-if="data.config" class="node-config">
              <div v-for="(value, key) in data.config" :key="key" class="config-item">
                <span class="config-key">{{ key }}:</span>
                <span class="config-value">{{ value }}</span>
              </div>
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
import '@vue-flow/core/dist/style.css'
import '@vue-flow/controls/dist/style.css'

const props = defineProps<{
  nodes: any[]
  edges: any[]
}>()

const emit = defineEmits(['update:nodes', 'update:edges'])

const { addNodes, addEdges, project } = useVueFlow()

const elements = computed({
  get: () => [...props.nodes, ...props.edges],
  set: (val) => {
    const n = val.filter((el: any) => !el.source)
    const e = val.filter((el: any) => el.source)
    emit('update:nodes', n)
    emit('update:edges', e)
  }
})

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

  const nodeData = JSON.parse(data)
  const position = project({ x: event.clientX - 300, y: event.clientY - 100 })

  const newNode = {
    id: `node_${Date.now()}`,
    type: 'custom',
    position,
    data: {
      label: nodeData.label,
      kind: nodeData.kind,
      icon: nodeData.icon,
      type: nodeData.type,
      config: {}
    },
  }

  addNodes([newNode])
}

const onConnect = (connection: any) => {
  addEdges([{
    id: `edge_${Date.now()}`,
    source: connection.source,
    target: connection.target,
    animated: true,
  }])
}

const onNodeClick = (event: any, node: any) => {
  console.log('Node clicked:', node)
}
</script>

<style scoped>
.flow-canvas {
  flex: 1;
  height: 100%;
}

.custom-node {
  background: #fff;
  border: 1px solid #dcdfe6;
  border-radius: 8px;
  min-width: 150px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}

.node-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  background: #f5f7fa;
  border-radius: 8px 8px 0 0;
  font-weight: 500;
  font-size: 13px;
}

.node-body {
  padding: 8px 12px;
}

.node-config {
  font-size: 11px;
  color: #909399;
}

.config-item {
  display: flex;
  gap: 4px;
  margin-bottom: 2px;
}

.config-key {
  color: #606266;
}

.config-value {
  color: #409EFF;
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
