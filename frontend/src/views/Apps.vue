<template>
  <div class="apps-page">
    <div class="page-header">
      <h2>应用管理</h2>
      <el-button type="primary">
        <el-icon><Plus /></el-icon>新建应用
      </el-button>
    </div>

    <el-row :gutter="20">
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
import { ref } from 'vue'

const apps = ref([
  { 
    id: '1', 
    name: '主分拣中心', 
    description: '主仓库的分拣业务流程管理', 
    status: 'ACTIVE',
    createdAt: '2024-01-10'
  },
  { 
    id: '2', 
    name: '退货处理', 
    description: '退货商品的入库和分拣处理', 
    status: 'ACTIVE',
    createdAt: '2024-01-12'
  },
  { 
    id: '3', 
    name: '测试环境', 
    description: '测试和开发使用', 
    status: 'INACTIVE',
    createdAt: '2024-01-15'
  },
])

const getStatusType = (status: string) => {
  const types: Record<string, string> = {
    'ACTIVE': 'success',
    'INACTIVE': 'info',
    'SUSPENDED': 'warning',
  }
  return types[status] || 'info'
}
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
  background: #409EFF;
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
