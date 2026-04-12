<template>
  <div class="flows-page">
    <div class="page-header">
      <h2>工作流管理</h2>
      <el-button type="primary" @click="handleCreate">
        <el-icon><Plus /></el-icon>新建工作流
      </el-button>
    </div>

    <el-card>
      <el-table :data="flows" v-loading="loading" style="width: 100%">
        <el-table-column prop="name" label="名称" min-width="150">
          <template #default="{ row }">
            <div class="flow-name">
              <el-icon><Share /></el-icon>
              <span>{{ row.name }}</span>
            </div>
          </template>
        </el-table-column>
        <el-table-column prop="description" label="描述" min-width="200" show-overflow-tooltip />
        <el-table-column prop="version" label="版本" width="80" />
        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="getStatusType(row.status)">{{ row.status }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="updatedAt" label="更新时间" width="180" />
        <el-table-column label="操作" width="250" fixed="right">
          <template #default="{ row }">
            <el-button type="primary" link @click="handleEdit(row)">编辑</el-button>
            <el-button type="primary" link @click="handleExecute(row)">执行</el-button>
            <el-button type="danger" link @click="handleDelete(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- Create/Edit Dialog -->
    <el-dialog
      v-model="dialogVisible"
      :title="dialogTitle"
      width="600px"
    >
      <el-form :model="form" label-width="80px">
        <el-form-item label="名称">
          <el-input v-model="form.name" placeholder="请输入工作流名称" />
        </el-form-item>
        <el-form-item label="描述">
          <el-input v-model="form.description" type="textarea" placeholder="请输入描述" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSave">确定</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'

const router = useRouter()

interface Flow {
  id: string
  name: string
  description: string
  version: number
  status: string
  updatedAt: string
}

const loading = ref(false)
const flows = ref<Flow[]>([])
const dialogVisible = ref(false)
const dialogTitle = ref('')
const form = ref({
  id: '',
  name: '',
  description: '',
})

const fetchFlows = async () => {
  loading.value = true
  // Mock data
  flows.value = [
    { id: '1', name: '正向分拣流程', description: '标准的正向分拣业务处理流程', version: 1, status: 'PUBLISHED', updatedAt: '2024-01-15 10:30:00' },
    { id: '2', name: '格口异常处理', description: '处理格口满包、故障等异常情况', version: 2, status: 'DRAFT', updatedAt: '2024-01-15 09:20:00' },
    { id: '3', name: '波次管理流程', description: '波次创建、启动、关闭流程', version: 1, status: 'PUBLISHED', updatedAt: '2024-01-14 16:45:00' },
    { id: '4', name: '订单执行流程', description: '订单从接收到完成的全流程', version: 3, status: 'ARCHIVED', updatedAt: '2024-01-13 11:00:00' },
  ]
  loading.value = false
}

const getStatusType = (status: string) => {
  const types: Record<string, string> = {
    'PUBLISHED': 'success',
    'DRAFT': 'warning',
    'ARCHIVED': 'info',
  }
  return types[status] || 'info'
}

const handleCreate = () => {
  dialogTitle.value = '新建工作流'
  form.value = { id: '', name: '', description: '' }
  dialogVisible.value = true
}

const handleEdit = (row: Flow) => {
  router.push(`/flows/${row.id}/editor`)
}

const handleExecute = (row: Flow) => {
  ElMessage.success(`开始执行工作流: ${row.name}`)
}

const handleDelete = (row: Flow) => {
  ElMessageBox.confirm(
    `确定要删除工作流 "${row.name}" 吗？`,
    '提示',
    { confirmButtonText: '确定', cancelButtonText: '取消', type: 'warning' }
  ).then(() => {
    ElMessage.success('删除成功')
  })
}

const handleSave = () => {
  dialogVisible.value = false
  ElMessage.success('保存成功')
}

onMounted(() => {
  fetchFlows()
})
</script>

<style scoped>
.flows-page {
  padding: 20px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.flow-name {
  display: flex;
  align-items: center;
  gap: 8px;
}
</style>
