<template>
  <div class="integration">
    <h1>外部系统对接</h1>
    
    <!-- Integration Status Overview -->
    <el-row :gutter="20" class="status-row">
      <el-col :span="6">
        <el-card class="integration-card rcs" :class="{ 'disconnected': !rcsStatus.connected }">
          <div class="card-icon">
            <el-icon><Cpu /></el-icon>
          </div>
          <div class="card-info">
            <div class="title">RCS 机器人系统</div>
            <div class="status">
              <el-tag :type="rcsStatus.connected ? 'success' : 'danger'">
                {{ rcsStatus.connected ? '已连接' : '未连接' }}
              </el-tag>
              <span class="latency" v-if="rcsStatus.connected">{{ rcsStatus.latency }}ms</span>
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
            <div class="title">工作站 SSE</div>
            <div class="status">
              <el-tag type="success">{{ sseStatus.station }}个连接</el-tag>
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

    <!-- RCS Configuration -->
    <el-row :gutter="20" class="section-row">
      <el-col :span="12">
        <el-card>
          <template #header>
            <div class="section-header">
              <span>RCS 配置</span>
              <el-button type="primary" size="small" @click="testRcsConnection">
                测试连接
              </el-button>
            </div>
          </template>
          <el-form :model="rcsConfig" label-width="120px">
            <el-form-item label="RCS服务地址">
              <el-input v-model="rcsConfig.baseUrl" placeholder="http://localhost:12300">
                <template #append>
                  <el-button @click="saveRcsConfig">保存</el-button>
                </template>
              </el-input>
            </el-form-item>
            <el-form-item label="超时时间">
              <el-input-number v-model="rcsConfig.timeout" :min="1000" :max="60000" :step="1000" />
              <span class="unit">ms</span>
            </el-form-item>
            <el-form-item label="重试次数">
              <el-input-number v-model="rcsConfig.retryCount" :min="1" :max="10" />
            </el-form-item>
          </el-form>
          
          <el-divider />
          
          <h4>RCS操作</h4>
          <el-row :gutter="10" class="operation-row">
            <el-col :span="8">
              <el-button type="primary" @click="batchWallOnline" size="small">播种墙上线</el-button>
            </el-col>
            <el-col :span="8">
              <el-button type="danger" @click="batchWallOffline" size="small">播种墙下线</el-button>
            </el-col>
            <el-col :span="8">
              <el-button @click="updatePlatform" size="small">更新地图</el-button>
            </el-col>
          </el-row>
          <el-row :gutter="10" class="operation-row">
            <el-col :span="8">
              <el-button @click="switchChute" size="small">切换格口</el-button>
            </el-col>
            <el-col :span="8">
              <el-button @click="dispatchTask" size="small">下发任务</el-button>
            </el-col>
            <el-col :span="8">
              <el-button @click="cancelTask" size="small">取消任务</el-button>
            </el-col>
          </el-row>
        </el-card>
      </el-col>
      
      <el-col :span="12">
        <el-card>
          <template #header>
            <div class="section-header">
              <span>SSE 连接管理</span>
              <el-button type="primary" size="small" @click="refreshSseStatus">
                刷新
              </el-button>
            </div>
          </template>
          
          <el-row :gutter="20">
            <el-col :span="12">
              <div class="sse-stat">
                <div class="stat-label">工作站连接</div>
                <div class="stat-value">{{ sseStatus.station }}</div>
              </div>
            </el-col>
            <el-col :span="12">
              <div class="sse-stat">
                <div class="stat-label">RCS连接</div>
                <div class="stat-value">{{ sseStatus.rcs }}</div>
              </div>
            </el-col>
          </el-row>
          
          <el-divider />
          
          <h4>连接测试</h4>
          <el-form :model="sseTestForm" label-width="100px">
            <el-form-item label="类型">
              <el-radio-group v-model="sseTestForm.type">
                <el-radio label="station">工作站</el-radio>
                <el-radio label="rcs">RCS</el-radio>
              </el-radio-group>
            </el-form-item>
            <el-form-item label="ID">
              <el-input v-model="sseTestForm.id" placeholder="工作站ID或RCS ID" />
            </el-form-item>
            <el-form-item label="平台" v-if="sseTestForm.type === 'station'">
              <el-input v-model="sseTestForm.platformId" placeholder="平台ID" />
            </el-form-item>
            <el-form-item>
              <el-button type="primary" @click="testSseConnection">建立SSE连接</el-button>
              <el-button @click="sendTestEvent">发送测试事件</el-button>
            </el-form-item>
          </el-form>
        </el-card>
      </el-col>
    </el-row>

    <!-- Task Dispatch Monitor - 任务分配监控 -->
    <el-row :gutter="20" class="section-row">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="section-header">
              <span>任务分配监控</span>
              <div class="header-actions">
                <el-button type="primary" size="small" @click="simulateDispatch">模拟分配</el-button>
                <el-button size="small" @click="refreshDispatchMonitor">刷新</el-button>
              </div>
            </div>
          </template>
          
          <el-row :gutter="20" class="dispatch-stats">
            <el-col :span="4">
              <div class="dispatch-stat">
                <div class="label">待分配订单</div>
                <div class="value">{{ dispatchStats.pending }}</div>
              </div>
            </el-col>
            <el-col :span="4">
              <div class="dispatch-stat">
                <div class="label">已分配</div>
                <div class="value success">{{ dispatchStats.assigned }}</div>
              </div>
            </el-col>
            <el-col :span="4">
              <div class="dispatch-stat">
                <div class="label">空闲格口</div>
                <div class="value">{{ dispatchStats.availableChutes }}</div>
              </div>
            </el-col>
            <el-col :span="4">
              <div class="dispatch-stat">
                <div class="label">占用格口</div>
                <div class="value warning">{{ dispatchStats.occupiedChutes }}</div>
              </div>
            </el-col>
            <el-col :span="4">
              <div class="dispatch-stat">
                <div class="label">成功分配率</div>
                <div class="value">{{ dispatchStats.successRate }}%</div>
              </div>
            </el-col>
            <el-col :span="4">
              <div class="dispatch-stat">
                <div class="label">平均耗时</div>
                <div class="value">{{ dispatchStats.avgTime }}ms</div>
              </div>
            </el-col>
          </el-row>
          
          <el-divider />
          
          <el-table :data="dispatchLogs" style="width: 100%" max-height="300">
            <el-table-column prop="time" label="时间" width="100" />
            <el-table-column prop="orderId" label="订单号" width="150" show-overflow-tooltip />
            <el-table-column prop="agvId" label="车辆" width="100" />
            <el-table-column prop="stationId" label="工作站" width="100" />
            <el-table-column prop="chuteId" label="分配格口" width="100" />
            <el-table-column prop="status" label="状态" width="80">
              <template #default="scope">
                <el-tag :type="scope.row.status === 'success' ? 'success' : 'danger'" size="small">
                  {{ scope.row.status === 'success' ? '成功' : '失败' }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="duration" label="耗时" width="80">
              <template #default="scope">
                {{ scope.row.duration }}ms
              </template>
            </el-table-column>
            <el-table-column prop="message" label="消息" show-overflow-tooltip />
          </el-table>
        </el-card>
      </el-col>
    </el-row>

    <!-- Order-Chute Binding Management - 订单格口绑定管理 -->
    <el-row :gutter="20" class="section-row">
      <el-col :span="12">
        <el-card>
          <template #header>
            <div class="section-header">
              <span>订单-格口绑定管理</span>
              <el-button type="danger" size="small" @click="releaseAllBindings">释放全部</el-button>
            </div>
          </template>
          <el-table :data="bindings" style="width: 100%" max-height="300" v-loading="bindingLoading">
            <el-table-column prop="order_id" label="订单号" width="120" show-overflow-tooltip />
            <el-table-column prop="chute_id" label="格口号" width="80" />
            <el-table-column prop="platform_id" label="平台" width="80" />
            <el-table-column prop="status" label="状态" width="80">
              <template #default="scope">
                <el-tag :type="scope.row.status === 'ACTIVE' ? 'success' : 'info'" size="small">
                  {{ scope.row.status === 'ACTIVE' ? '绑定' : '释放' }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column label="操作" width="80">
              <template #default="scope">
                <el-button 
                  v-if="scope.row.status === 'ACTIVE'"
                  type="danger" 
                  size="small"
                  @click="releaseBinding(scope.row.order_id)"
                >
                  释放
                </el-button>
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-col>
      
      <el-col :span="12">
        <el-card>
          <template #header>
            <div class="section-header">
              <span>格口状态</span>
              <el-button size="small" @click="refreshChuteStatus">刷新</el-button>
            </div>
          </template>
          <el-row :gutter="10">
            <el-col :span="8" v-for="chute in chuteStatus" :key="chute.chute_id">
              <div class="chute-card" :class="chute.status.toLowerCase()">
                <div class="chute-id">{{ chute.chute_id }}</div>
                <div class="chute-status">
                  <el-tag :type="getChuteTagType(chute.status)" size="small">
                    {{ chute.status }}
                  </el-tag>
                </div>
                <div class="chute-order" v-if="chute.order_id">
                  订单: {{ chute.order_id.substring(0, 8) }}...
                </div>
              </div>
            </el-col>
          </el-row>
        </el-card>
      </el-col>
    </el-row>

    <!-- Station Operations -->
    <el-row :gutter="20" class="section-row">
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
                <el-option label="发车" value="depart" />
                <el-option label="返岗" value="returnToWork" />
                <el-option label="离岗" value="leaveWork" />
              </el-select>
            </el-form-item>
            <el-form-item label="条码" v-if="stationForm.operation === 'scan'">
              <el-input v-model="stationForm.barcode" placeholder="请输入条码" />
            </el-form-item>
            <el-form-item label="订单" v-if="stationForm.operation === 'depart'">
              <el-select v-model="stationForm.orderId" multiple placeholder="选择订单">
                <el-option v-for="order in availableOrders" :key="order" :label="order" :value="order" />
              </el-select>
            </el-form-item>
            <el-form-item>
              <el-button type="primary" @click="executeStationOperation">执行</el-button>
            </el-form-item>
          </el-form>
        </el-card>
      </el-col>
      
      <el-col :span="12">
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
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Cpu, Cellphone, Monitor, Box } from '@element-plus/icons-vue'
import axios from 'axios'

const API_BASE = '/api/v1'

// Status
const rcsStatus = ref({ connected: true, latency: 15 })
const pdaCount = ref(5)
const sseStatus = ref({ station: 0, rcs: 0 })
const miniSorterCount = ref(3)
const bindingLoading = ref(false)
const availableOrders = ref(['ORDER001', 'ORDER002', 'ORDER003'])

// Forms
const rcsConfig = ref({
  baseUrl: 'http://localhost:12300',
  timeout: 10000,
  retryCount: 3
})

const sseTestForm = ref({
  type: 'station',
  id: '',
  platformId: ''
})

const stationForm = ref({
  stationId: '',
  operation: 'getTask',
  barcode: '',
  orderId: []
})

// Data
const bindings = ref([])
const chuteStatus = ref([])
const apiLogs = ref([])

const dispatchStats = ref({
  pending: 12,
  assigned: 45,
  availableChutes: 20,
  occupiedChutes: 15,
  successRate: 98.5,
  avgTime: 35
})

const dispatchLogs = ref([
  { time: '10:23:15', orderId: 'ORD001', agvId: 'AGV001', stationId: 'ST001', chuteId: 'CH005', status: 'success', duration: 32, message: '分配成功' },
  { time: '10:23:12', orderId: 'ORD002', agvId: 'AGV002', stationId: 'ST002', chuteId: 'CH008', status: 'success', duration: 28, message: '分配成功' },
  { time: '10:23:08', orderId: 'ORD003', agvId: 'AGV003', stationId: 'ST001', chuteId: '', status: 'error', duration: 45, message: '无空闲格口' }
])

// Add log
const addLog = (method, path, status) => {
  apiLogs.value.unshift({
    time: new Date().toLocaleTimeString(),
    method,
    path,
    status,
    type: status >= 200 && status < 300 ? 'success' : 'error'
  })
  if (apiLogs.value.length > 50) apiLogs.value.pop()
}

// Clear log
const clearLog = () => { apiLogs.value = [] }

// RCS Functions
const testRcsConnection = async () => {
  try {
    const start = Date.now()
    const response = await axios.get(`${API_BASE}/rcs/health`, { timeout: 5000 })
    rcsStatus.value.latency = Date.now() - start
    rcsStatus.value.connected = response.data.code === 0
    ElMessage.success(`RCS连接正常，延迟 ${rcsStatus.value.latency}ms`)
  } catch (error) {
    rcsStatus.value.connected = false
    ElMessage.error('RCS连接失败')
  }
}

const saveRcsConfig = async () => {
  try {
    await axios.post(`${API_BASE}/config/rcs`, rcsConfig.value)
    ElMessage.success('RCS配置保存成功')
  } catch (error) {
    ElMessage.error('保存失败')
  }
}

const batchWallOnline = async () => {
  try {
    const response = await axios.post(`${API_BASE}/wall/batch/online`, {
      data: [{ mc_id: 'WALL001', chute_id: 'CH001', rfid: 1001, wall_location: 'L' }]
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

const batchWallOffline = async () => {
  try {
    const response = await axios.post(`${API_BASE}/wall/batch/offline`, { data: [1001] })
    if (response.data.code === 0) {
      ElMessage.success('批量下线成功')
      addLog('POST', '/wall/batch/offline', 200)
    }
  } catch (error) {
    ElMessage.error('批量下线失败')
  }
}

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
  }
}

const switchChute = async () => {
  try {
    const response = await axios.post(`${API_BASE}/model/equipment/batch/switch`, {
      model_id: 'PLAT001',
      switch_infos: [{ equipment_name: 'CH001', equipment_type: 'CHUTE', target_status: 'OPEN' }]
    })
    if (response.data.code === 0) {
      ElMessage.success('格口切换成功')
    }
  } catch (error) {
    ElMessage.error('格口切换失败')
  }
}

const dispatchTask = async () => {
  try {
    const response = await axios.post(`${API_BASE}/task/dispatch`, {
      task_id: 'TASK_' + Date.now(),
      task_type: 'DEPART_FLIP',
      task_params: {
        order_id: 'ORDER001',
        agv_id: 'AGV001',
        platform_id: 'PLAT001',
        start_code: 'ST001',
        target_code: 'CH001'
      }
    })
    if (response.data.code === 0) {
      ElMessage.success('任务下发成功')
      addLog('POST', '/task/dispatch', 200)
    }
  } catch (error) {
    ElMessage.error('任务下发失败')
  }
}

const cancelTask = async () => {
  try {
    const response = await axios.post(`${API_BASE}/task/cancel`, { task_id: 'TASK001' })
    if (response.data.code === 0) ElMessage.success('任务取消成功')
  } catch (error) {
    ElMessage.error('任务取消失败')
  }
}

// SSE Functions
const refreshSseStatus = async () => {
  try {
    const response = await axios.get(`${API_BASE}/sse/status`)
    if (response.data.code === 0) {
      sseStatus.value = response.data.data
    }
  } catch (error) {
    console.error('获取SSE状态失败:', error)
  }
}

const testSseConnection = () => {
  const { type, id, platformId } = sseTestForm.value
  if (!id) {
    ElMessage.warning('请输入ID')
    return
  }
  
  let url = `${API_BASE}/sse/station/operation/connect/${id}`
  if (type === 'rcs') {
    url = `${API_BASE}/sse/rcs/operation/sse/subscribe?rcs_id=${id}&platform_ids=PLAT001`
  } else {
    url += `?platform_id=${platformId || 'PLAT001'}`
  }
  
  const eventSource = new EventSource(url)
  eventSource.onopen = () => {
    ElMessage.success(`${type === 'station' ? '工作站' : 'RCS'} SSE连接成功`)
  }
  eventSource.onmessage = (event) => {
    console.log('SSE Message:', event.data)
  }
  eventSource.onerror = () => {
    ElMessage.error('SSE连接错误')
    eventSource.close()
  }
}

const sendTestEvent = async () => {
  try {
    await axios.post(`${API_BASE}/sse/test`, {
      target: sseTestForm.value.type === 'station' ? 'Station' : 'AllRcs',
      target_id: sseTestForm.value.id,
      event_type: 'test'
    })
    ElMessage.success('测试事件已发送')
  } catch (error) {
    ElMessage.error('发送失败')
  }
}

// Task Dispatch Functions
const simulateDispatch = () => {
  const newLog = {
    time: new Date().toLocaleTimeString(),
    orderId: 'ORD' + Math.floor(Math.random() * 1000),
    agvId: 'AGV00' + Math.floor(Math.random() * 10),
    stationId: 'ST00' + Math.floor(Math.random() * 3),
    chuteId: 'CH00' + Math.floor(Math.random() * 20),
    status: Math.random() > 0.1 ? 'success' : 'error',
    duration: Math.floor(Math.random() * 50) + 20,
    message: Math.random() > 0.1 ? '分配成功' : '无空闲格口'
  }
  dispatchLogs.value.unshift(newLog)
  if (dispatchLogs.value.length > 20) dispatchLogs.value.pop()
  
  if (newLog.status === 'success') {
    dispatchStats.value.assigned++
    dispatchStats.value.occupiedChutes++
    dispatchStats.value.availableChutes--
  }
}

const refreshDispatchMonitor = () => {
  ElMessage.success('任务分配监控已刷新')
}

// Binding Functions
const fetchBindings = async () => {
  bindingLoading.value = true
  try {
    const response = await axios.get(`${API_BASE}/order-chute-bindings`)
    if (response.data.code === 0) {
      bindings.value = response.data.data || []
    }
  } catch (error) {
    console.error('获取绑定数据失败:', error)
  } finally {
    bindingLoading.value = false
  }
}

const releaseBinding = async (orderId) => {
  try {
    await axios.post(`${API_BASE}/order-chute-bindings/release`, { order_id: orderId })
    ElMessage.success('绑定已释放')
    fetchBindings()
  } catch (error) {
    ElMessage.error('释放失败')
  }
}

const releaseAllBindings = async () => {
  try {
    await ElMessageBox.confirm('确定要释放所有绑定吗？', '警告', { type: 'warning' })
    await axios.post(`${API_BASE}/order-chute-bindings/release-all`)
    ElMessage.success('全部绑定已释放')
    fetchBindings()
  } catch (error) {
    if (error !== 'cancel') ElMessage.error('释放失败')
  }
}

// Chute Functions
const refreshChuteStatus = async () => {
  try {
    const response = await axios.get(`${API_BASE}/chutes/status`)
    if (response.data.code === 0) {
      chuteStatus.value = response.data.data || []
    }
  } catch (error) {
    // Mock data
    chuteStatus.value = Array.from({ length: 12 }, (_, i) => ({
      chute_id: `CH${String(i + 1).padStart(3, '0')}`,
      status: ['OPEN', 'CLOSE', 'OCCUPIED'][Math.floor(Math.random() * 3)],
      order_id: Math.random() > 0.5 ? `ORDER${Math.floor(Math.random() * 100)}` : null
    }))
  }
}

const getChuteTagType = (status) => {
  const types = { OPEN: 'success', CLOSE: 'info', OCCUPIED: 'warning', FORBIDDEN: 'danger' }
  return types[status] || 'info'
}

// Station Functions
const executeStationOperation = async () => {
  const operations = {
    getTask: '/station/operation/getTaskInfo',
    scan: '/station/operation/scanBarcode',
    depart: '/station/operation/robotDeparture',
    returnToWork: '/station/operation/returnToWork',
    leaveWork: '/station/operation/leaveWork'
  }
  
  try {
    let payload = { station_id: stationForm.value.stationId }
    if (stationForm.value.operation === 'scan') {
      payload.barcode = stationForm.value.barcode
    } else if (stationForm.value.operation === 'depart') {
      payload.order_ids = stationForm.value.orderId
      payload.wave_id = 'WAVE001'
    }
    
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

// Mini Sorter Functions
const miniWallOnline = async () => {
  try {
    await axios.post(`${API_BASE}/mini/wall/online`, { mc_id: 'MINI001', chute_id: 'CH001', rfid: 1001, wall_location: 'L' })
    ElMessage.success('播种墙上线成功')
  } catch (error) {
    ElMessage.error('播种墙上线失败')
  }
}

const miniWallOffline = async () => {
  try {
    await axios.post(`${API_BASE}/mini/wall/offline`, { rfid: 1001 })
    ElMessage.success('播种墙下线成功')
  } catch (error) {
    ElMessage.error('播种墙下线失败')
  }
}

const miniItemReady = async () => {
  try {
    await axios.post(`${API_BASE}/mini/item/ready`, { station_id: 'ST001', exist: true })
    ElMessage.success('货道就绪通知成功')
  } catch (error) {
    ElMessage.error('货道就绪通知失败')
  }
}

const miniDeviceReady = async () => {
  try {
    await axios.post(`${API_BASE}/mini/device/status`, { status: 1 })
    ElMessage.success('设备就绪通知成功')
  } catch (error) {
    ElMessage.error('设备就绪通知失败')
  }
}

const miniAgvChange = async () => {
  try {
    await axios.post(`${API_BASE}/mini/agv/change`, { station_id: 'ST001', agv_id: 'AGV001' })
    ElMessage.success('车辆变更成功')
  } catch (error) {
    ElMessage.error('车辆变更失败')
  }
}

const miniPing = async () => {
  try {
    await axios.post(`${API_BASE}/mini/ping`)
    ElMessage.success('心跳正常')
  } catch (error) {
    ElMessage.error('心跳失败')
  }
}

onMounted(() => {
  fetchBindings()
  refreshChuteStatus()
  refreshSseStatus()
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
    
    .el-icon { color: white; }
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
      display: flex;
      align-items: center;
      gap: 10px;
      
      .latency {
        font-size: 12px;
        color: #909399;
      }
    }
  }
  
  &.rcs .card-icon { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); }
  &.rcs.disconnected .card-icon { background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); }
  &.pda .card-icon { background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); }
  &.station .card-icon { background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%); }
  &.mini .card-icon { background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%); }
}

.section-row {
  margin-bottom: 20px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  
  .header-actions {
    display: flex;
    gap: 10px;
  }
}

.operation-row {
  margin-bottom: 10px;
}

.unit {
  margin-left: 8px;
  color: #909399;
}

.sse-stat {
  text-align: center;
  padding: 15px;
  background: #f5f7fa;
  border-radius: 8px;
  
  .stat-label {
    font-size: 12px;
    color: #909399;
    margin-bottom: 8px;
  }
  
  .stat-value {
    font-size: 24px;
    font-weight: bold;
    color: #409EFF;
  }
}

.dispatch-stats {
  margin-bottom: 20px;
  
  .dispatch-stat {
    text-align: center;
    padding: 15px;
    background: #f5f7fa;
    border-radius: 8px;
    
    .label {
      font-size: 12px;
      color: #909399;
      margin-bottom: 8px;
    }
    
    .value {
      font-size: 24px;
      font-weight: bold;
      color: #303133;
      
      &.success { color: #67C23A; }
      &.warning { color: #E6A23C; }
    }
  }
}

.chute-card {
  background: #f5f7fa;
  border-radius: 6px;
  padding: 10px;
  margin-bottom: 10px;
  text-align: center;
  border: 2px solid transparent;
  
  &.open { border-color: #67C23A; }
  &.occupied { border-color: #E6A23C; }
  &.close { border-color: #909399; }
  
  .chute-id {
    font-weight: bold;
    font-size: 14px;
    margin-bottom: 5px;
  }
  
  .chute-status {
    margin-bottom: 5px;
  }
  
  .chute-order {
    font-size: 11px;
    color: #909399;
  }
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
    
    .time { color: #888; min-width: 80px; }
    .method { color: #9cdcfe; min-width: 60px; }
    .path { color: #ce9178; flex: 1; }
    .status {
      min-width: 40px;
      text-align: right;
      &.success { color: #7ee787; }
      &.error { color: #f85149; }
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
