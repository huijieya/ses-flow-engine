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
import { ElMessage, ElMessageBox } from 'element-plus'
import NodeLibrary from '../components/NodeLibrary.vue'
import FlowCanvas from '../components/FlowCanvas.vue'
import PropertyPanel from '../components/PropertyPanel.vue'
import { getFlow, createFlow, updateFlow, executeFlow } from '@/api/flows'

const route = useRoute()
const router = useRouter()

const flowId = ref('')
const flowName = ref('未命名工作流')
const flowStatus = ref('DRAFT')
const nodes = ref<any[]>([])
const edges = ref<any[]>([])
const selectedNode = ref<any>(null)
const canvasRef = ref<any>(null)
const isNew = ref(false)
const isSaving = ref(false)

const onDragStart = (event: DragEvent, nodeType: string) => {
  if (event.dataTransfer) {
    event.dataTransfer.setData('application/vueflow', nodeType)
    event.dataTransfer.effectAllowed = 'move'
  }
}

const handleValidate = () => {
  // 简单验证：检查是否有开始和结束节点
  const hasStart = nodes.value.some((n: any) => n.type === 'start')
  const hasEnd = nodes.value.some((n: any) => n.type === 'end')
  
  if (!hasStart) {
    ElMessage.warning('工作流缺少开始节点')
    return false
  }
  if (!hasEnd) {
    ElMessage.warning('工作流缺少结束节点')
    return false
  }
  if (nodes.value.length > 0 && edges.value.length === 0) {
    ElMessage.warning('工作流节点之间缺少连接')
    return false
  }
  
  ElMessage.success('工作流验证通过')
  return true
}

const handleSave = async () => {
  // 验证工作流
  const isValid = handleValidate()
  if (!isValid) return
  
  isSaving.value = true
  try {
    // 构建 flow_json
    const flowDefinition = {
      nodes: nodes.value.map((n: any) => ({
        id: n.id,
        node_def_id: n.type || 'default',
        name: n.label || n.type || '节点',
        kind: n.data?.kind || 'device',
        position: { x: n.position?.x || 0, y: n.position?.y || 0 },
        config: n.data || {},
      })),
      edges: edges.value.map((e: any) => ({
        id: e.id,
        source: e.source,
        target: e.target,
        condition: e.data?.condition || null,
      })),
      variables: {},
    }
    
    if (isNew.value) {
      // 新建工作流
      const name = await ElMessageBox.prompt('请输入工作流名称', '新建工作流', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        inputValue: flowName.value,
      })
      
      if (!name.value) {
        ElMessage.warning('工作流名称不能为空')
        return
      }
      
      const result = await createFlow({
        name: name.value,
        description: flowName.value === '未命名工作流' ? '' : flowName.value,
        flow_json: flowDefinition,
        is_template: false,
      }) as any
      
      if (result.data && result.data.id) {
        flowId.value = result.data.id
        isNew.value = false
        flowName.value = name.value
        ElMessage.success('工作流创建成功')
        // 更新 URL
        router.replace(`/flows/${result.data.id}/editor`)
      }
    } else {
      // 更新工作流
      await updateFlow(flowId.value, {
        name: flowName.value,
        flow_json: flowDefinition,
      })
      ElMessage.success('工作流保存成功')
    }
  } catch (error: any) {
    if (error !== 'cancel') {
      console.error('保存工作流失败:', error)
      ElMessage.error('保存失败: ' + (error.message || '未知错误'))
    }
  } finally {
    isSaving.value = false
  }
}

const handleExecute = async () => {
  if (isNew.value) {
    ElMessage.warning('请先保存工作流')
    return
  }
  
  try {
    await executeFlow(flowId.value)
    ElMessage.success('开始执行工作流')
  } catch (error) {
    console.error('执行工作流失败:', error)
    ElMessage.error('执行失败')
  }
}

const loadFlow = async (id: string) => {
  try {
    const result = await getFlow(id) as any
    if (result.data) {
      flowName.value = result.data.name || '未命名工作流'
      flowStatus.value = result.data.status || 'DRAFT'
      
      // 解析 flow_json
      const flowDef = result.data.flow_json
      if (flowDef) {
        nodes.value = (flowDef.nodes || []).map((n: any) => ({
          id: n.id,
          type: n.node_def_id,
          label: n.name,
          position: n.position,
          data: n.config,
        }))
        edges.value = (flowDef.edges || []).map((e: any) => ({
          id: e.id,
          source: e.source,
          target: e.target,
          data: { condition: e.condition },
        }))
      }
    }
  } catch (error) {
    console.error('加载工作流失败:', error)
    ElMessage.error('加载工作流失败')
  }
}

onMounted(() => {
  const id = route.params.id as string
  flowId.value = id
  
  if (id === 'new') {
    isNew.value = true
    flowName.value = '未命名工作流'
    flowStatus.value = 'DRAFT'
    // 初始化一个空的工作流，可以添加默认的开始节点
    nodes.value = [
      {
        id: 'start',
        type: 'start',
        label: '开始',
        position: { x: 100, y: 100 },
        data: { kind: 'system' },
      },
      {
        id: 'end',
        type: 'end',
        label: '结束',
        position: { x: 400, y: 100 },
        data: { kind: 'system' },
      },
    ]
  } else {
    isNew.value = false
    loadFlow(id)
  }
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
