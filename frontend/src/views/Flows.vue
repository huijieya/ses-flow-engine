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
import { getFlows, createFlow, updateFlow, deleteFlow, executeFlow } from '@/api/flows'

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
const isEdit = ref(false)
const form = ref({
  id: '',
  name: '',
  description: '',
})

// 获取工作流列表
const fetchFlows = async () => {
  loading.value = true
  try {
    const res = await getFlows() as any
    if (res.data && Array.isArray(res.data)) {
      flows.value = res.data.map((item: any) => ({
        id: item.id,
        name: item.name,
        description: item.description || '',
        version: item.version || 1,
        status: item.status || 'DRAFT',
        updatedAt: item.updated_at || item.updatedAt || new Date().toISOString(),
      }))
    } else {
      // 后端未完全实现时使用空数组
      flows.value = []
    }
  } catch (error) {
    console.error('获取工作流列表失败:', error)
    ElMessage.warning('后端接口尚未完全实现，显示为空列表')
    flows.value = []
  } finally {
    loading.value = false
  }
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
  isEdit.value = false
  form.value = { id: '', name: '', description: '' }
  dialogVisible.value = true
}

const handleEdit = (row: Flow) => {
  router.push(`/flows/${row.id}/editor`)
}

const handleExecute = async (row: Flow) => {
  try {
    await executeFlow(row.id)
    ElMessage.success(`开始执行工作流: ${row.name}`)
  } catch (error) {
    console.error('执行工作流失败:', error)
    ElMessage.error('执行工作流失败')
  }
}

const handleDelete = async (row: Flow) => {
  try {
    await ElMessageBox.confirm(
      `确定要删除工作流 "${row.name}" 吗？`,
      '提示',
      { confirmButtonText: '确定', cancelButtonText: '取消', type: 'warning' }
    )
    await deleteFlow(row.id)
    ElMessage.success('删除成功')
    fetchFlows()
  } catch (error: any) {
    if (error !== 'cancel') {
      console.error('删除工作流失败:', error)
      ElMessage.error('删除失败')
    }
  }
}

const handleSave = async () => {
  try {
    if (!form.value.name) {
      ElMessage.warning('请输入工作流名称')
      return
    }
    
    if (isEdit.value && form.value.id) {
      await updateFlow(form.value.id, {
        name: form.value.name,
        description: form.value.description,
      })
    } else {
      await createFlow({
        name: form.value.name,
        description: form.value.description,
      })
    }
    
    dialogVisible.value = false
    ElMessage.success('保存成功')
    fetchFlows()
  } catch (error) {
    console.error('保存工作流失败:', error)
    ElMessage.error('保存失败')
  }
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
