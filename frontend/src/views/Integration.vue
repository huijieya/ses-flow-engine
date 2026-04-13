<template>
  <div class="integration">
    <h1>外部系统对接</h1>
    
    <!-- Integration Status Overview -->
    <el-row :gutter="20" class="status-row">
      <el-col :span="6">
        <el-card class="integration-card rcs">
          <div class="card-icon">
            <el-icon><Cpu /></el-icon>
          </div>
          <div class="card-info">
            <div class="title">RCS 机器人系统</div>
            <div class="status">
              <el-tag type="success">已连接</el-tag>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="integration-card pda">
          <div class="card-icon">
            <el-icon><Cellphone /></el-icon>
          </div>
          <div class="card-info">
            <div class="title">PDA 手持终端</div>
            <div class="status">
              <el-tag type="success">{{ pdaCount }}台在线</el-tag>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="integration-card station">
          <div class="card-icon">
            <el-icon><Monitor /></el-icon>
          </div>
          <div class="card-info">
            <div class="title">工作站</div>
            <div class="status">
              <el-tag type="success">{{ stationCount }}个在线</el-tag>
            </div>
          </div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="integration-card mini">
          <div class="card-icon">
            <el-icon><Box /></el-icon>
          </div>
          <div class="card-info">
            <div class="title">迷你分拣机</div>
            <div class="status">
              <el-tag type="warning">{{ miniSorterCount }}台运行中</el-tag>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- RCS Operations -->
    <el-row :gutter="20" class="section-row">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="section-header">
              <span>RCS 操作</span>
              <el-button type="primary" size="small" @click="refreshRcs">刷新</el-button>
            </div>
          </template>
          <el-row :gutter="10">
            <el-col :span="4">
              <el-button type="primary" @click="batchWallOnline">批量播种墙上线</el-button>
            </el-col>
            <el-col :span="4">
              <el-button type="danger" @click="batchWallOffline">批量播种墙下线</el-button>
            </el-col>
            <el-col :span="4">
              <el-button @click="updatePlatform">更新地图</el-button>
            </el-col>
            <el-col :span="4">
              <el-button @click="robotRequestFlip">请求卸货</el-button>
            </el-col>
            <el-col :span="4">
              <el-button @click="arrivedStation">车辆到达</el-button>
            </el-col>
            <el-col :span="4">
              <el-button @click="leaveStation">车辆离开</el-button>
            </el-col>
          </el-row>
          
          <el-divider />
          
          <h4>播种墙状态</h4>
          <el-table :data="wallList" style="width: 100%">
            <el-table-column prop="wall_id" label="墙ID" />
            <el-table-column prop="rfid" label="RFID" />
            <el-table-column prop="status" label="状态">
              <template #default="scope">
                <el-tag :type="scope.row.status === 'online' ? 'success' : 'info'">
                  {{ scope.row.status }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="location" label="位置" />
          </el-table>
        </el-card>
      </el-col>
    </el-row>

    <!-- PDA Operations -->
    <el-row :gutter="20" class="section-row">
      <el-col :span="12">
        <el-card>
          <template #header>
            <div class="section-header">
              <span>PDA 操作</span>
            </div>
          </template>
          <el-form :model="pdaForm" label-width="100px">
            <el-form-item label="操作类型">
              <el-radio-group v-model="pdaForm.operation">
                <el-radio label="pack">封包</el-radio>
                <el-radio label="distribute">播种</el-radio>
                <el-radio label="bindBox">绑箱</el-radio>
              </el-radio-group>
            </el-form-item>
            <el-form-item label="目标ID">
              <el-input v-model="pdaForm.targetId" placeholder="请输入目标ID" />
            </el-form-item>
            <el-form-item label="SKU">
              <el-input v-model="pdaForm.sku" placeholder="请输入SKU" v-if="pdaForm.operation === 'distribute'" />
            </el-form-item>
            <el-form-item>
              <el-button type="primary" @click="executePdaOperation">执行</el-button>
            </el-form-item>
          </el-form>
        </el-card>
      </el-col>
      <el-col :span="12">
        <el-card>
          <template #header>
            <div class="section-header">
              <span>工作站操作</span>
            </div>
          </template>
          <el-form :model="stationForm" label-width="100px">
            <el-form-item label="工作站ID">
              <el-input v-model="stationForm.stationId" placeholder="请输入工作站ID" />
            </el-form-item>
            <el-form-item label="操作">
              <el-select v-model="stationForm.operation">
                <el-option label="获取任务" value="getTask" />
                <el-option label="扫码" value="scan" />
                <el-option label="返岗" value="returnToWork" />
                <el-option label="离岗" value="leaveWork" />
              </el-select>
            </el-form-item>
            <el-form-item>
              <el-button type="primary" @click="executeStationOperation">执行</el-button>
            </el-form-item>
          </el-form>
        </el-card>
      </el-col>
    </el-row>

    <!-- Mini Sorter Operations -->
    <el-row class="section-row">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="section-header">
              <span>迷你分拣机操作</span>
            </div>
          </template>
          <el-row :gutter="10">
            <el-col :span="4">
              <el-button @click="miniWallOnline">播种墙上线</el-button>
            </el-col>
            <el-col :span="4">
              <el-button @click="miniWallOffline">播种墙下线</el-button>
            </el-col>
            <el-col :span="4">
              <el-button @click="miniItemReady">货道就绪</el-button>
            </el-col>
            <el-col :span="4">
              <el-button @click="miniDeviceReady">设备就绪</el-button>
            </el-col>
            <el-col :span="4">
              <el-button @click="miniAgvChange">车辆变更</el-button>
            </el-col>
            <el-col :span="4">
              <el-button @click="miniPing">心跳</el-button>
            </el-col>
          </el-row>
        </el-card>
      </el-col>
    </el-row>

    <!-- API Test Console -->
    <el-row class="section-row">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="section-header">
              <span>API 测试控制台</span>
              <el-button type="primary" size="small" @click="clearLog">清空日志</el-button>
            </div>
          </template>
          <div class="api-log">
            <div v-for="(log, index) in apiLogs" :key="index" class="log-item" :class="log.type">
              <span class="time">{{ log.time }}</span>
              <span class="method">{{ log.method }}</span>
              <span class="path">{{ log.path }}</span>
              <span class="status" :class="log.status >= 200 && log.status < 300 ? 'success' : 'error'">
                {{ log.status }}
              </span>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { Cpu, Cellphone, Monitor, Box } from '@element-plus/icons-vue'
import axios from 'axios'

const API_BASE = '/api'

const pdaCount = ref(5)
const stationCount = ref(8)
const miniSorterCount = ref(3)
const wallList = ref([])
const apiLogs = ref([])

const pdaForm = ref({
  operation: 'pack',
  targetId: '',
  sku: ''
})

const stationForm = ref({
  stationId: '',
  operation: 'getTask'
})

// Add log
const addLog = (method, path, status) => {
  apiLogs.value.unshift({
    time: new Date().toLocaleTimeString(),
    method,
    path,
    status,
    type: status >= 200 && status < 300 ? 'success' : 'error'
  })
  if (apiLogs.value.length > 50) {
    apiLogs.value.pop()
  }
}

// Clear log
const clearLog = () => {
  apiLogs.value = []
}

// Refresh RCS status
const refreshRcs = () => {
  ElMessage.info('刷新RCS状态')
}

// Batch wall online
const batchWallOnline = async () => {
  try {
    const response = await axios.post(`${API_BASE}/wall/batch/online`, {
      data: [
        { mc_id: 'WALL001', chute_id: 'CH001', rfid: 1001, wall_location: 'L' }
      ]
    })
    if (response.data.code === 0) {
      ElMessage.success('批量上线成功')
      addLog('POST', '/wall/batch/online', 200)
    }
  } catch (error) {
    ElMessage.error('批量上线失败')
    addLog('POST', '/wall/batch/online', error.response?.status || 500)
  }
}

// Batch wall offline
const batchWallOffline = async () => {
  try {
    const response = await axios.post(`${API_BASE}/wall/batch/offline`, {
      data: [1001]
    })
    if (response.data.code === 0) {
      ElMessage.success('批量下线成功')
      addLog('POST', '/wall/batch/offline', 200)
    }
  } catch (error) {
    ElMessage.error('批量下线失败')
    addLog('POST', '/wall/batch/offline', error.response?.status || 500)
  }
}

// Update platform
const updatePlatform = async () => {
  try {
    const response = await axios.post(`${API_BASE}/rcs/operation/updatePlatform`, [{
      platform_id: 'PLAT001',
      platform_name: '平台1',
      grids: []
    }])
    if (response.data.code === 0) {
      ElMessage.success('更新地图成功')
      addLog('POST', '/rcs/operation/updatePlatform', 200)
    }
  } catch (error) {
    ElMessage.error('更新地图失败')
    addLog('POST', '/rcs/operation/updatePlatform', error.response?.status || 500)
  }
}

// Robot request flip
const robotRequestFlip = async () => {
  try {
    const response = await axios.post(`${API_BASE}/rcs/operation/robotRequestFlip`, {
      task_id: 'TASK001',
      target_id: 'TARGET001'
    })
    if (response.data.code === 0) {
      ElMessage.success('请求卸货成功')
      addLog('POST', '/rcs/operation/robotRequestFlip', 200)
    }
  } catch (error) {
    ElMessage.error('请求卸货失败')
    addLog('POST', '/rcs/operation/robotRequestFlip', error.response?.status || 500)
  }
}

// Arrived station
const arrivedStation = async () => {
  try {
    const response = await axios.post(`${API_BASE}/rcs/operation/arrived/station`, {
      agv_id: 'AGV001',
      station_id: 'STATION001'
    })
    if (response.data.code === 0) {
      ElMessage.success('车辆到达成功')
      addLog('POST', '/rcs/operation/arrived/station', 200)
    }
  } catch (error) {
    ElMessage.error('车辆到达失败')
    addLog('POST', '/rcs/operation/arrived/station', error.response?.status || 500)
  }
}

// Leave station
const leaveStation = async () => {
  try {
    const response = await axios.post(`${API_BASE}/rcs/operation/leave/station`, {
      agv_id: 'AGV001',
      station_id: 'STATION001'
    })
    if (response.data.code === 0) {
      ElMessage.success('车辆离开成功')
      addLog('POST', '/rcs/operation/leave/station', 200)
    }
  } catch (error) {
    ElMessage.error('车辆离开失败')
    addLog('POST', '/rcs/operation/leave/station', error.response?.status || 500)
  }
}

// Execute PDA operation
const executePdaOperation = async () => {
  const operations = {
    pack: '/pda/pack',
    distribute: '/pda/manDistribute',
    bindBox: '/pda/bindBox'
  }
  
  try {
    const payload = pdaForm.value.operation === 'distribute' 
      ? { target_id: pdaForm.value.targetId, sku: pdaForm.value.sku }
      : { target_id: pdaForm.value.targetId }
      
    const response = await axios.post(`${API_BASE}${operations[pdaForm.value.operation]}`, payload)
    if (response.data.code === 0) {
      ElMessage.success('操作成功')
      addLog('POST', operations[pdaForm.value.operation], 200)
    }
  } catch (error) {
    ElMessage.error('操作失败')
    addLog('POST', operations[pdaForm.value.operation], error.response?.status || 500)
  }
}

// Execute station operation
const executeStationOperation = async () => {
  const operations = {
    getTask: '/station/operation/getTaskInfo',
    scan: '/station/operation/scanBarcode',
    returnToWork: '/station/operation/returnToWork',
    leaveWork: '/station/operation/leaveWork'
  }
  
  try {
    const payload = { station_id: stationForm.value.station_id }
    const response = await axios.post(`${API_BASE}${operations[stationForm.value.operation]}`, payload)
    if (response.data.code === 0) {
      ElMessage.success('操作成功')
      addLog('POST', operations[stationForm.value.operation], 200)
    }
  } catch (error) {
    ElMessage.error('操作失败')
    addLog('POST', operations[stationForm.value.operation], error.response?.status || 500)
  }
}

// Mini sorter operations
const miniWallOnline = async () => {
  try {
    const response = await axios.post(`${API_BASE}/mini/wall/online`, {
      mc_id: 'MINI001', chute_id: 'CH001', rfid: 1001, wall_location: 'L'
    })
    if (response.data.code === 0) {
      ElMessage.success('播种墙上线成功')
      addLog('POST', '/mini/wall/online', 200)
    }
  } catch (error) {
    ElMessage.error('播种墙上线失败')
    addLog('POST', '/mini/wall/online', error.response?.status || 500)
  }
}

const miniWallOffline = async () => {
  try {
    const response = await axios.post(`${API_BASE}/mini/wall/offline`, { rfid: 1001 })
    if (response.data.code === 0) {
      ElMessage.success('播种墙下线成功')
      addLog('POST', '/mini/wall/offline', 200)
    }
  } catch (error) {
    ElMessage.error('播种墙下线失败')
    addLog('POST', '/mini/wall/offline', error.response?.status || 500)
  }
}

const miniItemReady = async () => {
  try {
    const response = await axios.post(`${API_BASE}/mini/item/ready`, {
      station_id: 'ST001', exist: true
    })
    if (response.data.code === 0) {
      ElMessage.success('货道就绪通知成功')
      addLog('POST', '/mini/item/ready', 200)
    }
  } catch (error) {
    ElMessage.error('货道就绪通知失败')
    addLog('POST', '/mini/item/ready', error.response?.status || 500)
  }
}

const miniDeviceReady = async () => {
  try {
    const response = await axios.post(`${API_BASE}/mini/device/status`, { status: 1 })
    if (response.data.code === 0) {
      ElMessage.success('设备就绪通知成功')
      addLog('POST', '/mini/device/status', 200)
    }
  } catch (error) {
    ElMessage.error('设备就绪通知失败')
    addLog('POST', '/mini/device/status', error.response?.status || 500)
  }
}

const miniAgvChange = async () => {
  try {
    const response = await axios.post(`${API_BASE}/mini/agv/change`, {
      station_id: 'ST001', agv_id: 'AGV001'
    })
    if (response.data.code === 0) {
      ElMessage.success('车辆变更成功')
      addLog('POST', '/mini/agv/change', 200)
    }
  } catch (error) {
    ElMessage.error('车辆变更失败')
    addLog('POST', '/mini/agv/change', error.response?.status || 500)
  }
}

const miniPing = async () => {
  try {
    const response = await axios.post(`${API_BASE}/mini/ping`)
    if (response.data.code === 0) {
      ElMessage.success('心跳正常')
      addLog('POST', '/mini/ping', 200)
    }
  } catch (error) {
    ElMessage.error('心跳失败')
    addLog('POST', '/mini/ping', error.response?.status || 500)
  }
}

onMounted(() => {
  // Initialize with mock data
  wallList.value = [
    { wall_id: 'WALL001', rfid: 1001, status: 'online', location: 'A-01' },
    { wall_id: 'WALL002', rfid: 1002, status: 'online', location: 'A-02' },
    { wall_id: 'WALL003', rfid: 1003, status: 'offline', location: 'A-03' }
  ]
})
</script>

<style scoped>
.integration {
  padding: 20px;
}

.status-row {
  margin-bottom: 20px;
}

.integration-card {
  display: flex;
  align-items: center;
  padding: 10px;
  
  .card-icon {
    width: 60px;
    height: 60px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 32px;
    margin-right: 15px;
    
    .el-icon {
      color: white;
    }
  }
  
  .card-info {
    flex: 1;
    
    .title {
      font-size: 16px;
      font-weight: bold;
      color: #303133;
      margin-bottom: 5px;
    }
    
    .status {
      font-size: 14px;
    }
  }
  
  &.rcs .card-icon {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  }
  
  &.pda .card-icon {
    background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
  }
  
  &.station .card-icon {
    background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
  }
  
  &.mini .card-icon {
    background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);
  }
}

.section-row {
  margin-bottom: 20px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.api-log {
  height: 300px;
  overflow-y: auto;
  background: #1e1e1e;
  border-radius: 4px;
  padding: 10px;
  font-family: 'Courier New', monospace;
  font-size: 12px;
  
  .log-item {
    padding: 5px 0;
    border-bottom: 1px solid #333;
    display: flex;
    gap: 10px;
    
    .time {
      color: #888;
      min-width: 80px;
    }
    
    .method {
      color: #9cdcfe;
      min-width: 60px;
    }
    
    .path {
      color: #ce9178;
      flex: 1;
    }
    
    .status {
      min-width: 40px;
      text-align: right;
      
      &.success {
        color: #7ee787;
      }
      
      &.error {
        color: #f85149;
      }
    }
  }
}

h1 {
  margin-bottom: 20px;
  color: #303133;
}

h4 {
  margin: 15px 0 10px;
  color: #606266;
}
</style>
