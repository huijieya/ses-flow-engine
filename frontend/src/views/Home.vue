<template>
  <div class="home">
    <el-row :gutter="20">
      <el-col :span="6">
        <el-card class="stat-card">
          <div class="stat-icon blue">
            <el-icon><Share /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-value">{{ stats.flows }}</div>
            <div class="stat-label">工作流</div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="stat-card">
          <div class="stat-icon green">
            <el-icon><CircleCheck /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-value">{{ stats.instances }}</div>
            <div class="stat-label">运行实例</div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="stat-card">
          <div class="stat-icon orange">
            <el-icon><Cpu /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-value">{{ stats.devices }}</div>
            <div class="stat-label">在线设备</div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="stat-card">
          <div class="stat-icon purple">
            <el-icon><Document /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-value">{{ stats.nodes }}</div>
            <div class="stat-label">节点类型</div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" class="mt-20">
      <el-col :span="12">
        <el-card title="最近执行的流程">
          <template #header>
            <div class="card-header">
              <span>最近执行的流程</span>
              <el-button type="primary" link>查看全部</el-button>
            </div>
          </template>
          <el-table :data="recentFlows" style="width: 100%">
            <el-table-column prop="name" label="流程名称" />
            <el-table-column prop="status" label="状态">
              <template #default="{ row }">
                <el-tag :type="getStatusType(row.status)">{{ row.status }}</el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="createdAt" label="执行时间" />
          </el-table>
        </el-card>
      </el-col>
      <el-col :span="12">
        <el-card>
          <template #header>
            <div class="card-header">
              <span>系统状态</span>
            </div>
          </template>
          <div class="system-status">
            <div class="status-item">
              <span class="status-label">API 服务</span>
              <el-tag type="success">运行中</el-tag>
            </div>
            <div class="status-item">
              <span class="status-label">数据库</span>
              <el-tag type="success">已连接</el-tag>
            </div>
            <div class="status-item">
              <span class="status-label">Redis</span>
              <el-tag type="success">已连接</el-tag>
            </div>
            <div class="status-item">
              <span class="status-label">事件总线</span>
              <el-tag type="success">运行中</el-tag>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const stats = ref({
  flows: 12,
  instances: 45,
  devices: 8,
  nodes: 24,
})

const recentFlows = ref([
  { name: '正向分拣流程', status: 'COMPLETED', createdAt: '2024-01-15 10:30:00' },
  { name: '格口异常处理', status: 'RUNNING', createdAt: '2024-01-15 10:25:00' },
  { name: '波次管理流程', status: 'COMPLETED', createdAt: '2024-01-15 10:20:00' },
])

const getStatusType = (status: string) => {
  const types: Record<string, string> = {
    'COMPLETED': 'success',
    'RUNNING': 'primary',
    'FAILED': 'danger',
    'PENDING': 'warning',
  }
  return types[status] || 'info'
}
</script>

<style scoped>
.stat-card {
  display: flex;
  align-items: center;
  padding: 10px;
}

.stat-icon {
  width: 60px;
  height: 60px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 28px;
  color: #fff;
  margin-right: 16px;
}

.stat-icon.blue {
  background-color: #409EFF;
}

.stat-icon.green {
  background-color: #67C23A;
}

.stat-icon.orange {
  background-color: #E6A23C;
}

.stat-icon.purple {
  background-color: #8E44AD;
}

.stat-value {
  font-size: 24px;
  font-weight: bold;
  color: #303133;
}

.stat-label {
  font-size: 14px;
  color: #909399;
  margin-top: 4px;
}

.mt-20 {
  margin-top: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.system-status {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.status-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background-color: #f5f7fa;
  border-radius: 4px;
}

.status-label {
  font-size: 14px;
  color: #606266;
}
</style>
