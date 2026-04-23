<template>
  <div class="apps-page">
    <div class="page-header">
      <h2>应用管理</h2>
      <el-button type="primary" @click="handleCreate">
        <el-icon><Plus /></el-icon>新建应用
      </el-button>
    </div>

    <el-row :gutter="20" v-loading="loading">
      <el-col :span="8" v-for="app in apps" :key="app.id">
        <el-card class="app-card" shadow="hover">
          <div class="app-header">
            <div class="app-icon">
              <el-icon :size="32"><Monitor /></el-icon>
            </div>
            <div class="app-info">
              <h4>{{ app.name }}</h4>
              <el-tag :type="getStatusType(app.status)" size="small">{{ app.status }}</el-tag>
            </div>
          </div>
          <p class="app-description">{{ app.description }}</p>
          <div class="app-meta">
            <span>创建于 {{ app.createdAt }}</span>
          </div>
          <div class="app-actions">
            <el-button type="primary" link>进入</el-button>
            <el-button link>设置</el-button>
            <el-button type="danger" link>删除</el-button>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { getApps } from '@/api/apps'

interface App {
  id: string
  name: string
  description: string
  status: string
  createdAt: string
}

const loading = ref(false)
const apps = ref<App[]>([])

const fetchApps = async () => {
  loading.value = true
  try {
    const res = await getApps() as any
    if (res.data && Array.isArray(res.data)) {
      apps.value = res.data.map((item: any) => ({
        id: item.id,
        name: item.name,
        description: item.description || '',
        status: item.status || 'ACTIVE',
        createdAt: item.created_at || item.createdAt || '-',
      }))
    } else {
      apps.value = []
    }
  } catch (error) {
    console.error('获取应用列表失败:', error)
    ElMessage.warning('后端接口尚未完全实现，显示为空列表')
    apps.value = []
  } finally {
    loading.value = false
  }
}

const getStatusType = (status: string) => {
  const types: Record<string, string> = {
    'ACTIVE': 'success',
    'INACTIVE': 'info',
    'SUSPENDED': 'warning',
  }
  return types[status] || 'info'
}

const handleCreate = () => {
  ElMessage.info('新建应用功能待实现')
}

onMounted(() => {
  fetchApps()
})
</script>

<style scoped>
.apps-page {
  padding: 20px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.app-card {
  margin-bottom: 20px;
}

.app-header {
  display: flex;
  gap: 16px;
  margin-bottom: 12px;
}

.app-icon {
  width: 48px;
  height: 48px;
  background: rgba(46, 198, 214, 1);
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
}

.app-info h4 {
  margin: 0 0 8px 0;
}

.app-description {
  color: #606266;
  font-size: 14px;
  margin: 12px 0;
  min-height: 40px;
}

.app-meta {
  font-size: 12px;
  color: #909399;
  margin-bottom: 12px;
}

.app-actions {
  display: flex;
  gap: 16px;
}
</style>
