<template>
  <div class="flow-editor">
    <div class="editor-header">
      <div class="header-left">
        <el-button @click="router.back()">
          <el-icon><ArrowLeft /></el-icon>
        </el-button>
        <h3>{{ flowName }}</h3>
        <el-tag :type="flowStatus === 'PUBLISHED' ? 'success' : 'warning'">
          {{ flowStatus }}
        </el-tag>
      </div>
      <div class="header-right">
        <el-button @click="handleValidate">
          <el-icon><Check /></el-icon>验证
        </el-button>
        <el-button type="primary" @click="handleSave">
          <el-icon><DocumentChecked /></el-icon>保存
        </el-button>
        <el-button type="success" @click="handleExecute">
          <el-icon><VideoPlay /></el-icon>执行
        </el-button>
      </div>
    </div>

    <div class="editor-body">
      <NodeLibrary @drag-start="onDragStart" />
      <FlowCanvas
        ref="canvasRef"
        :nodes="nodes"
        :edges="edges"
        @update:nodes="nodes = $event"
        @update:edges="edges = $event"
      />
      <PropertyPanel :selected-node="selectedNode" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import NodeLibrary from '../components/NodeLibrary.vue'
import FlowCanvas from '../components/FlowCanvas.vue'
import PropertyPanel from '../components/PropertyPanel.vue'

const route = useRoute()
const router = useRouter()

const flowName = ref('未命名工作流')
const flowStatus = ref('DRAFT')
const nodes = ref<any[]>([])
const edges = ref<any[]>([])
const selectedNode = ref<any>(null)
const canvasRef = ref<any>(null)

const onDragStart = (event: DragEvent, nodeType: string) => {
  if (event.dataTransfer) {
    event.dataTransfer.setData('application/vueflow', nodeType)
    event.dataTransfer.effectAllowed = 'move'
  }
}

const handleValidate = () => {
  ElMessage.success('工作流验证通过')
}

const handleSave = () => {
  console.log('Saving flow:', { nodes: nodes.value, edges: edges.value })
  ElMessage.success('工作流保存成功')
}

const handleExecute = () => {
  ElMessage.success('开始执行工作流')
}

onMounted(() => {
  const flowId = route.params.id as string
  console.log('Loading flow:', flowId)
})
</script>

<style scoped>
.flow-editor {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.editor-header {
  height: 56px;
  background: #fff;
  border-bottom: 1px solid #dcdfe6;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.header-left h3 {
  margin: 0;
}

.header-right {
  display: flex;
  gap: 8px;
}

.editor-body {
  flex: 1;
  display: flex;
  overflow: hidden;
}
</style>
