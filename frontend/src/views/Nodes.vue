<template>
  <div class="nodes-page">
    <div class="page-header">
      <h2>节点库</h2>
      <el-button type="primary">
        <el-icon><Plus /></el-icon>新建节点
      </el-button>
    </div>

    <el-card v-loading="loading">
      <el-alert
        v-if="nodes.length === 0 && !loading"
        title="暂无节点定义，请启动后端服务并执行数据库迁移以加载系统内置节点"
        type="info"
        :closable="false"
        style="margin-bottom: 16px"
      />
      <el-tabs v-model="activeTab">
        <el-tab-pane v-for="kind in nodeKinds" :key="kind.value" :label="kind.label" :name="kind.value">
          <el-table :data="getNodesByKind(kind.value)" style="width: 100%">
            <el-table-column prop="name" label="名称" min-width="150">
              <template #default="{ row }">
                <div style="display: flex; align-items: center; gap: 8px">
                  <el-icon :size="18" :color="row.color || '#2EC6D6'">
                    <component :is="row.icon || 'CircleCheck'" />
                  </el-icon>
                  <span>{{ row.name }}</span>
                </div>
              </template>
            </el-table-column>
            <el-table-column prop="nodeId" label="节点ID" width="150" />
            <el-table-column prop="description" label="描述" min-width="250" show-overflow-tooltip />
            <el-table-column prop="category" label="分类" width="120" />
            <el-table-column label="系统节点" width="100">
              <template #default="{ row }">
                <el-tag v-if="row.isSystem" type="success" size="small">是</el-tag>
                <el-tag v-else type="info" size="small">否</el-tag>
              </template>
            </el-table-column>
          </el-table>
        </el-tab-pane>
      </el-tabs>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { getNodeDefinitions } from '@/api/nodes'

const activeTab = ref('device')
const loading = ref(false)
const nodeKinds = [
  { label: '设备节点', value: 'device' },
  { label: '逻辑节点', value: 'logic' },
  { label: '数据节点', value: 'data' },
  { label: '查询节点', value: 'query' },
  { label: '系统节点', value: 'system' },
]

interface NodeDefinition {
  id: string
  nodeId: string
  name: string
  description: string
  kind: string
  category: string
  icon?: string
  color?: string
  isSystem?: boolean
}

const nodes = ref<NodeDefinition[]>([])

const getNodesByKind = (kind: string) => {
  return nodes.value.filter(n => n.kind === kind)
}

const fetchNodes = async () => {
  loading.value = true
  try {
    const res = await getNodeDefinitions() as any
    if (res.data && Array.isArray(res.data)) {
      nodes.value = res.data.map((item: any) => ({
        id: item.id,
        nodeId: item.node_id || item.nodeId,
        name: item.name,
        description: item.description || '',
        kind: item.kind || 'device',
        category: item.category || 'general',
        icon: item.icon,
        color: item.color,
        isSystem: item.is_system || item.isSystem,
      }))
    } else {
      nodes.value = []
    }
  } catch (error) {
    console.error('获取节点定义列表失败:', error)
    ElMessage.warning('后端接口尚未完全实现，显示为空列表')
    nodes.value = []
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchNodes()
})
</script>

<style scoped>
.nodes-page {
  padding: 20px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}
</style>
