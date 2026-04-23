<template>
  <div class="devices-page">
    <div class="page-header">
      <h2>设备管理</h2>
      <el-button type="primary">
        <el-icon><Plus /></el-icon>添加设备
      </el-button>
    </div>

    <el-row :gutter="20">
      <el-col :span="16">
        <el-card v-loading="loading">
          <el-table :data="devices" style="width: 100%">
            <el-table-column prop="deviceId" label="设备ID" min-width="120" />
            <el-table-column prop="deviceName" label="名称" min-width="150" />
            <el-table-column prop="deviceType" label="类型" width="100" />
            <el-table-column prop="online" label="状态" width="80">
              <template #default="{ row }">
                <el-tag :type="row.online ? 'success' : 'danger'">
                  {{ row.online ? '在线' : '离线' }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="lastHeartbeat" label="最后心跳" width="180" />
            <el-table-column label="操作" width="150">
              <template #default="{ row }">
                <el-button type="primary" link>详情</el-button>
                <el-button type="danger" link>删除</el-button>
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-col>
      <el-col :span="8">
        <el-card>
          <template #header>
            <span>设备概览</span>
          </template>
          <div class="device-stats">
            <div class="stat-item">
              <span class="stat-label">总设备数</span>
              <span class="stat-value">{{ devices.length }}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">在线设备</span>
              <span class="stat-value online">{{ onlineCount }}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">离线设备</span>
              <span class="stat-value offline">{{ offlineCount }}</span>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { getDevices } from '@/api/devices'

interface Device {
  id: string
  deviceId: string
  deviceName: string
  deviceType: string
  online: boolean
  lastHeartbeat: string
}

const loading = ref(false)
const devices = ref<Device[]>([])

const onlineCount = computed(() => devices.value.filter(d => d.online).length)
const offlineCount = computed(() => devices.value.filter(d => !d.online).length)

const fetchDevices = async () => {
  loading.value = true
  try {
    const res = await getDevices() as any
    if (res.data && Array.isArray(res.data)) {
      devices.value = res.data.map((item: any) => ({
        id: item.id,
        deviceId: item.device_id || item.deviceId,
        deviceName: item.device_name || item.deviceName,
        deviceType: item.device_type || item.deviceType,
        online: item.online || false,
        lastHeartbeat: item.last_heartbeat || item.lastHeartbeat || '-',
      }))
    } else {
      devices.value = []
    }
  } catch (error) {
    console.error('获取设备列表失败:', error)
    ElMessage.warning('后端接口尚未完全实现，显示为空列表')
    devices.value = []
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchDevices()
})
</script>

<style scoped>
.devices-page {
  padding: 20px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.device-stats {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.stat-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: #f5f7fa;
  border-radius: 4px;
}

.stat-label {
  color: #606266;
}

.stat-value {
  font-size: 20px;
  font-weight: bold;
  color: #303133;
}

.stat-value.online {
  color: #67C23A;
}

.stat-value.offline {
  color: #F56C6C;
}
</style>
