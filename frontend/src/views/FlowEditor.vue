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
        <el-tag v-if="flowType" type="info" size="small" class="flow-type-tag">
          {{ flowType }}
        </el-tag>
      </div>
      <div class="header-right">
        <el-button @click="handleValidate">
          <el-icon><Check /></el-icon>验证
        </el-button>
        <el-button type="primary" @click="handleSave">
          <el-icon><DocumentChecked /></el-icon>保存
        </el-button>
        <el-button type="success" @click="handleExecute" :disabled="isNew">
          <el-icon><VideoPlay /></el-icon>执行
        </el-button>
        <el-dropdown @command="handleTemplateCommand">
          <el-button>
            模板<el-icon class="el-icon--right"><ArrowDown /></el-icon>
          </el-button>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="wave_process">波次分拣流程</el-dropdown-item>
              <el-dropdown-item command="station_task">工作站任务流程</el-dropdown-item>
              <el-dropdown-item command="rcs_dispatch">RCS调度流程</el-dropdown-item>
              <el-dropdown-item command="task_dispatch">任务分配流程</el-dropdown-item>
              <el-dropdown-item command="clear">清空画布</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
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
        @node-select="onNodeSelect"
        @node-deselect="onNodeDeselect"
      />
      <PropertyPanel 
        :selected-node="selectedNode" 
        :node-definition="selectedNodeDefinition"
        @update:config="onConfigUpdate"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { ArrowLeft, Check, DocumentChecked, VideoPlay, ArrowDown } from '@element-plus/icons-vue'
import NodeLibrary from '../components/NodeLibrary.vue'
import FlowCanvas from '../components/FlowCanvas.vue'
import PropertyPanel from '../components/PropertyPanel.vue'
import { getFlow, createFlow, updateFlow, executeFlow } from '@/api/flows'
import { getNodeDefinitions } from '@/api/nodes'

interface NodeDefinition {
  node_id: string
  name: string
  description?: string
  kind: string
  config_schema?: any
  default_config?: any
  icon?: string
  color?: string
}

interface FlowNode {
  id: string
  type: string
  label: string
  position: { x: number; y: number }
  data: {
    label: string
    kind: string
    nodeId: string
    icon?: string
    color?: string
    config?: any
  }
}

const route = useRoute()
const router = useRouter()

const flowId = ref('')
const flowName = ref('未命名工作流')
const flowStatus = ref('DRAFT')
const flowType = ref('')
const nodes = ref<FlowNode[]>([])
const edges = ref<any[]>([])
const selectedNode = ref<any>(null)
const canvasRef = ref<any>(null)
const isNew = ref(false)
const isSaving = ref(false)
const nodeDefinitions = ref<NodeDefinition[]>([])

// 获取选中的节点定义
const selectedNodeDefinition = computed(() => {
  if (!selectedNode.value) return undefined
  const nodeId = selectedNode.value.data?.nodeId
  return nodeDefinitions.value.find(def => def.node_id === nodeId)
})

// 加载节点定义
const loadNodeDefinitions = async () => {
  try {
    const result = await getNodeDefinitions() as any
    if (result.data) {
      nodeDefinitions.value = result.data
    }
  } catch (error) {
    console.error('加载节点定义失败:', error)
  }
}

const onDragStart = (event: DragEvent, nodeType: string) => {
  if (event.dataTransfer) {
    event.dataTransfer.setData('application/vueflow', nodeType)
    event.dataTransfer.effectAllowed = 'move'
  }
}

const onNodeSelect = (node: any) => {
  selectedNode.value = node
}

const onNodeDeselect = () => {
  selectedNode.value = null
}

// 处理属性更新
const onConfigUpdate = ({ nodeId, data }: { nodeId: string, data: any }) => {
  const nodeIndex = nodes.value.findIndex(n => n.id === nodeId)
  if (nodeIndex !== -1) {
    nodes.value[nodeIndex] = {
      ...nodes.value[nodeIndex],
      data: {
        ...nodes.value[nodeIndex].data,
        label: data.label,
        config: data.config
      }
    }
    if (canvasRef.value) {
      canvasRef.value.updateNodeData(nodeId, {
        label: data.label,
        config: data.config
      })
    }
  }
}

// 处理模板命令
const handleTemplateCommand = (command: string) => {
  switch (command) {
    case 'wave_process':
      loadWaveProcessTemplate()
      break
    case 'station_task':
      loadStationTaskTemplate()
      break
    case 'rcs_dispatch':
      loadRcsDispatchTemplate()
      break
    case 'task_dispatch':
      loadTaskDispatchTemplate()
      break
    case 'clear':
      clearCanvas()
      break
  }
}

// 波次分拣流程模板
const loadWaveProcessTemplate = () => {
  flowType.value = '波次分拣'
  const startDef = nodeDefinitions.value.find(def => def.node_id === 'start')
  const createDef = nodeDefinitions.value.find(def => def.node_id === 'wave_create')
  const startWaveDef = nodeDefinitions.value.find(def => def.node_id === 'wave_start')
  const assignDef = nodeDefinitions.value.find(def => def.node_id === 'order_assign')
  const closeDef = nodeDefinitions.value.find(def => def.node_id === 'wave_close')
  const endDef = nodeDefinitions.value.find(def => def.node_id === 'end')

  nodes.value = [
    { id: 'start', type: 'custom', label: '开始', position: { x: 100, y: 100 },
      data: { label: '开始', kind: 'system', nodeId: 'start', icon: startDef?.icon, color: startDef?.color, config: {} }},
    { id: 'wave_create', type: 'custom', label: '创建波次', position: { x: 300, y: 100 },
      data: { label: '创建波次', kind: 'business', nodeId: 'wave_create', icon: createDef?.icon, color: createDef?.color, config: { platform_id: '', wave_id: '' } }},
    { id: 'wave_start', type: 'custom', label: '启动波次', position: { x: 500, y: 100 },
      data: { label: '启动波次', kind: 'business', nodeId: 'wave_start', icon: startWaveDef?.icon, color: startWaveDef?.color, config: {} }},
    { id: 'order_assign', type: 'custom', label: '分配订单', position: { x: 700, y: 100 },
      data: { label: '分配订单', kind: 'business', nodeId: 'order_assign', icon: assignDef?.icon, color: assignDef?.color, config: {} }},
    { id: 'wave_close', type: 'custom', label: '关闭波次', position: { x: 900, y: 100 },
      data: { label: '关闭波次', kind: 'business', nodeId: 'wave_close', icon: closeDef?.icon, color: closeDef?.color, config: {} }},
    { id: 'end', type: 'custom', label: '结束', position: { x: 1100, y: 100 },
      data: { label: '结束', kind: 'system', nodeId: 'end', icon: endDef?.icon, color: endDef?.color, config: {} }}
  ]
  edges.value = [
    { id: 'e1', source: 'start', target: 'wave_create' },
    { id: 'e2', source: 'wave_create', target: 'wave_start' },
    { id: 'e3', source: 'wave_start', target: 'order_assign' },
    { id: 'e4', source: 'order_assign', target: 'wave_close' },
    { id: 'e5', source: 'wave_close', target: 'end' }
  ]
  ElMessage.success('已加载波次分拣流程模板')
}

// 工作站任务流程模板
const loadStationTaskTemplate = () => {
  flowType.value = '工作站任务'
  const startDef = nodeDefinitions.value.find(def => def.node_id === 'start')
  const getTaskDef = nodeDefinitions.value.find(def => def.node_id === 'station_get_task')
  const scanDef = nodeDefinitions.value.find(def => def.node_id === 'station_scan')
  const dispatchDef = nodeDefinitions.value.find(def => def.node_id === 'rcs_dispatch')
  const endDef = nodeDefinitions.value.find(def => def.node_id === 'end')

  nodes.value = [
    { id: 'start', type: 'custom', label: '开始', position: { x: 100, y: 100 },
      data: { label: '开始', kind: 'system', nodeId: 'start', icon: startDef?.icon, color: startDef?.color, config: {} }},
    { id: 'get_task', type: 'custom', label: '获取任务', position: { x: 300, y: 100 },
      data: { label: '获取任务', kind: 'station', nodeId: 'station_get_task', icon: getTaskDef?.icon, color: getTaskDef?.color, config: { station_id: '' } }},
    { id: 'scan', type: 'custom', label: '扫码', position: { x: 500, y: 100 },
      data: { label: '扫码', kind: 'station', nodeId: 'station_scan', icon: scanDef?.icon, color: scanDef?.color, config: {} }},
    { id: 'dispatch', type: 'custom', label: 'RCS下发', position: { x: 700, y: 100 },
      data: { label: 'RCS下发', kind: 'rcs', nodeId: 'rcs_dispatch', icon: dispatchDef?.icon, color: dispatchDef?.color, config: {} }},
    { id: 'end', type: 'custom', label: '结束', position: { x: 900, y: 100 },
      data: { label: '结束', kind: 'system', nodeId: 'end', icon: endDef?.icon, color: endDef?.color, config: {} }}
  ]
  edges.value = [
    { id: 'e1', source: 'start', target: 'get_task' },
    { id: 'e2', source: 'get_task', target: 'scan' },
    { id: 'e3', source: 'scan', target: 'dispatch' },
    { id: 'e4', source: 'dispatch', target: 'end' }
  ]
  ElMessage.success('已加载工作站任务流程模板')
}

// RCS调度流程模板
const loadRcsDispatchTemplate = () => {
  flowType.value = 'RCS调度'
  const startDef = nodeDefinitions.value.find(def => def.node_id === 'start')
  const arrivedDef = nodeDefinitions.value.find(def => def.node_id === 'rcs_arrived')
  const requestFlipDef = nodeDefinitions.value.find(def => def.node_id === 'rcs_request_flip')
  const switchGridDef = nodeDefinitions.value.find(def => def.node_id === 'rcs_switch_grid')
  const endDef = nodeDefinitions.value.find(def => def.node_id === 'end')

  nodes.value = [
    { id: 'start', type: 'custom', label: '开始', position: { x: 100, y: 100 },
      data: { label: '开始', kind: 'system', nodeId: 'start', icon: startDef?.icon, color: startDef?.color, config: {} }},
    { id: 'arrived', type: 'custom', label: '车辆到达', position: { x: 300, y: 50 },
      data: { label: '车辆到达', kind: 'rcs', nodeId: 'rcs_arrived', icon: arrivedDef?.icon, color: arrivedDef?.color, config: {} }},
    { id: 'request_flip', type: 'custom', label: '请求卸货', position: { x: 500, y: 50 },
      data: { label: '请求卸货', kind: 'rcs', nodeId: 'rcs_request_flip', icon: requestFlipDef?.icon, color: requestFlipDef?.color, config: {} }},
    { id: 'switch_grid', type: 'custom', label: '切换格口', position: { x: 300, y: 150 },
      data: { label: '切换格口', kind: 'rcs', nodeId: 'rcs_switch_grid', icon: switchGridDef?.icon, color: switchGridDef?.color, config: {} }},
    { id: 'end', type: 'custom', label: '结束', position: { x: 700, y: 100 },
      data: { label: '结束', kind: 'system', nodeId: 'end', icon: endDef?.icon, color: endDef?.color, config: {} }}
  ]
  edges.value = [
    { id: 'e1', source: 'start', target: 'arrived' },
    { id: 'e2', source: 'arrived', target: 'request_flip' },
    { id: 'e3', source: 'request_flip', target: 'end' },
    { id: 'e4', source: 'start', target: 'switch_grid' },
    { id: 'e5', source: 'switch_grid', target: 'end' }
  ]
  ElMessage.success('已加载RCS调度流程模板')
}

// 任务分配流程模板
const loadTaskDispatchTemplate = () => {
  flowType.value = '任务分配'
  const startDef = nodeDefinitions.value.find(def => def.node_id === 'start')
  const taskDispatchDef = nodeDefinitions.value.find(def => def.node_id === 'task_dispatch')
  const deductDef = nodeDefinitions.value.find(def => def.node_id === 'task_deduct')
  const bindDef = nodeDefinitions.value.find(def => def.node_id === 'destination_bind')
  const rcsDispatchDef = nodeDefinitions.value.find(def => def.node_id === 'rcs_dispatch')
  const endDef = nodeDefinitions.value.find(def => def.node_id === 'end')

  nodes.value = [
    { id: 'start', type: 'custom', label: '开始', position: { x: 100, y: 100 },
      data: { label: '开始', kind: 'system', nodeId: 'start', icon: startDef?.icon, color: startDef?.color, config: {} }},
    { id: 'task_dispatch', type: 'custom', label: '任务分配', position: { x: 300, y: 100 },
      data: { label: '任务分配', kind: 'task', nodeId: 'task_dispatch', icon: taskDispatchDef?.icon, color: taskDispatchDef?.color, config: {} }},
    { id: 'deduct', type: 'custom', label: '数量扣减', position: { x: 500, y: 50 },
      data: { label: '数量扣减', kind: 'task', nodeId: 'task_deduct', icon: deductDef?.icon, color: deductDef?.color, config: {} }},
    { id: 'bind', type: 'custom', label: '目的地绑定', position: { x: 500, y: 150 },
      data: { label: '目的地绑定', kind: 'task', nodeId: 'destination_bind', icon: bindDef?.icon, color: bindDef?.color, config: {} }},
    { id: 'rcs_dispatch', type: 'custom', label: 'RCS下发', position: { x: 700, y: 100 },
      data: { label: 'RCS下发', kind: 'rcs', nodeId: 'rcs_dispatch', icon: rcsDispatchDef?.icon, color: rcsDispatchDef?.color, config: {} }},
    { id: 'end', type: 'custom', label: '结束', position: { x: 900, y: 100 },
      data: { label: '结束', kind: 'system', nodeId: 'end', icon: endDef?.icon, color: endDef?.color, config: {} }}
  ]
  edges.value = [
    { id: 'e1', source: 'start', target: 'task_dispatch' },
    { id: 'e2', source: 'task_dispatch', target: 'deduct' },
    { id: 'e3', source: 'task_dispatch', target: 'bind' },
    { id: 'e4', source: 'deduct', target: 'rcs_dispatch' },
    { id: 'e5', source: 'bind', target: 'rcs_dispatch' },
    { id: 'e6', source: 'rcs_dispatch', target: 'end' }
  ]
  ElMessage.success('已加载任务分配流程模板')
}

// 清空画布
const clearCanvas = () => {
  ElMessageBox.confirm('确定要清空当前工作流吗？', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(() => {
    nodes.value = []
    edges.value = []
    flowType.value = ''
    ElMessage.success('画布已清空')
  }).catch(() => {})
}

const handleValidate = () => {
  const hasStart = nodes.value.some((n: any) => n.data?.nodeId === 'start')
  const hasEnd = nodes.value.some((n: any) => n.data?.nodeId === 'end')
  
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
  const isValid = handleValidate()
  if (!isValid) return
  
  isSaving.value = true
  try {
    const flowDefinition = {
      nodes: nodes.value.map((n: any) => ({
        id: n.id,
        node_def_id: n.data?.nodeId || 'default',
        name: n.data?.label || '节点',
        kind: n.data?.kind || 'device',
        position: { x: n.position?.x || 0, y: n.position?.y || 0 },
        config: n.data?.config || {},
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
        description: flowType.value || '',
        flow_json: flowDefinition,
        is_template: false,
      }) as any
      
      if (result.data && result.data.id) {
        flowId.value = result.data.id
        isNew.value = false
        flowName.value = name.value
        ElMessage.success('工作流创建成功')
        router.replace(`/flows/${result.data.id}/editor`)
      }
    } else {
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
      flowType.value = result.data.description || ''
      
      const flowDef = result.data.flow_json
      if (flowDef) {
        nodes.value = (flowDef.nodes || []).map((n: any) => {
          const nodeDef = nodeDefinitions.value.find(def => def.node_id === n.node_def_id)
          return {
            id: n.id,
            type: 'custom',
            label: n.name,
            position: n.position,
            data: {
              label: n.name,
              kind: n.kind,
              nodeId: n.node_def_id,
              icon: nodeDef?.icon,
              color: nodeDef?.color,
              config: n.config || {}
            },
          }
        })
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

onMounted(async () => {
  await loadNodeDefinitions()
  
  const id = route.params.id as string
  flowId.value = id
  
  if (id === 'new') {
    isNew.value = true
    flowName.value = '未命名工作流'
    flowStatus.value = 'DRAFT'
    const startNodeDef = nodeDefinitions.value.find(def => def.node_id === 'start')
    const endNodeDef = nodeDefinitions.value.find(def => def.node_id === 'end')
    
    nodes.value = [
      {
        id: 'start',
        type: 'custom',
        label: '开始',
        position: { x: 100, y: 100 },
        data: { 
          label: '开始', 
          kind: 'system', 
          nodeId: 'start',
          icon: startNodeDef?.icon || 'CircleCheck',
          color: startNodeDef?.color || '#67C23A',
          config: {}
        },
      },
      {
        id: 'end',
        type: 'custom',
        label: '结束',
        position: { x: 400, y: 100 },
        data: { 
          label: '结束', 
          kind: 'system', 
          nodeId: 'end',
          icon: endNodeDef?.icon || 'CircleCloseFilled',
          color: endNodeDef?.color || '#909399',
          config: {}
        },
      },
    ]
  } else {
    isNew.value = false
    await loadFlow(id)
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

.flow-type-tag {
  margin-left: 8px;
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
