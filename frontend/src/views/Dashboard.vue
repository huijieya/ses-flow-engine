<template>
  <div class="dashboard">
    <h1>SES 分拣系统概览</h1>
    
    <!-- System Status Cards - 系统状态概览 -->
    <el-row :gutter="20" class="status-row">
      <el-col :span="4">
        <el-card class="status-card">
          <div class="card-header">
            <span>今日波次</span>
            <el-tag type="success">+{{ systemStatus?.twave_qty_compare_yest || 0 }}%</el-tag>
          </div>
          <div class="card-value">{{ systemStatus?.twave_qty || 0 }}</div>
          <div class="card-label">已完成: {{ systemStatus?.twave_completed || 0 }}</div>
        </el-card>
      </el-col>
      <el-col :span="4">
        <el-card class="status-card">
          <div class="card-header">
            <span>分拣数量</span>
            <el-tag type="warning">实时</el-tag>
          </div>
          <div class="card-value">{{ systemStatus?.tsorting_qty || 0 }}</div>
          <div class="card-label">较昨日 +{{ systemStatus?.tsorting_qty_compare_yest || 0 }}</div>
        </el-card>
      </el-col>
      <el-col :span="4">
        <el-card class="status-card">
          <div class="card-header">
            <span>封包数量</span>
            <el-tag>累计</el-tag>
          </div>
          <div class="card-value">{{ systemStatus?.tpack_qty || 0 }}</div>
          <div class="card-label">较昨日 +{{ systemStatus?.tpack_qty_compare_yest || 0 }}</div>
        </el-card>
      </el-col>
      <el-col :span="4">
        <el-card class="status-card" :class="{ 'warning': !rcsConnected }">
          <div class="card-header">
            <span>RCS连接</span>
            <el-tag :type="rcsConnected ? 'success' : 'danger'">
              {{ rcsConnected ? '已连接' : '未连接' }}
            </el-tag>
          </div>
          <div class="card-value">{{ rcsLatency }}ms</div>
          <div class="card-label">{{ rcsConnected ? '心跳正常' : '检查RCS服务' }}</div>
        </el-card>
      </el-col>
      <el-col :span="4">
        <el-card class="status-card">
          <div class="card-header">
            <span>SSE连接</span>
            <el-tag type="success">在线</el-tag>
          </div>
          <div class="card-value">{{ sseConnections }}</div>
          <div class="card-label">工作站: {{ stationConnections }} / RCS: {{ rcsConnections }}</div>
        </el-card>
      </el-col>
      <el-col :span="4">
        <el-card class="status-card">
          <div class="card-header">
            <span>格口占用</span>
            <el-tag type="info">{{ chuteOccupancyRate }}%</el-tag>
          </div>
          <div class="card-value">{{ activeChutes }}/{{ totalChutes }}</div>
          <div class="card-label">已绑定: {{ boundOrders }} 订单</div>
        </el-card>
      </el-col>
    </el-row>

    <!-- Platform Status - 平台地图状态 -->
    <el-row :gutter="20" class="section-row">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="card-header-title">
              <span>平台地图状态</span>
              <el-button type="primary" size="small" @click="refreshPlatforms">
                <el-icon><Refresh /></el-icon>刷新
              </el-button>
            </div>
          </template>
          <el-row :gutter="20">
            <el-col v-for="platform in platforms" :key="platform.platform_id" :span="8">
              <div class="platform-card">
                <div class="platform-header">
                  <span class="platform-name">{{ platform.platform_name || platform.platform_id }}</span>
                  <el-tag :type="platform.status === 'ACTIVE' ? 'success' : 'info'" size="small">
                    {{ platform.status === 'ACTIVE' ? '作业中' : '空闲' }}
                  </el-tag>
                </div>
                <div class="platform-stats">
                  <div class="stat">
                    <span class="label">工作站</span>
                    <span class="value">{{ platform.station_count || 0 }}</span>
                  </div>
                  <div class="stat">
                    <span class="label">格口</span>
                    <span class="value">{{ platform.chute_count || 0 }}</span>
                  </div>
                  <div class="stat">
                    <span class="label">活跃波次</span>
                    <span class="value">{{ platform.active_wave_count || 0 }}</span>
                  </div>
                </div>
                <div class="platform-progress">
                  <span class="label">格口占用</span>
                  <el-progress :percentage="platform.occupancy_rate || 0" :color="getOccupancyColor" />
                </div>
              </div>
            </el-col>
          </el-row>
        </el-card>
      </el-col>
    </el-row>

    <!-- Efficiency Chart -->
    <el-row :gutter="20" class="chart-row">
      <el-col :span="16">
        <el-card>
          <template #header>
            <div class="card-header-title">
              <span>分拣效率趋势</span>
              <span class="avg-efficiency">
                平均效率: {{ efficiencyData?.avg_efficiency || 0 }}件/小时
              </span>
            </div>
          </template>
          <div ref="efficiencyChart" class="chart-container"></div>
        </el-card>
      </el-col>
      <el-col :span="8">
        <el-card>
          <template #header>
            <span>效率统计</span>
          </template>
          <div class="efficiency-stats">
            <div class="stat-item">
              <span class="label">最高效率</span>
              <span class="value high">{{ efficiencyData?.high_efficiency || 0 }}</span>
            </div>
            <div class="stat-item">
              <span class="label">平均效率</span>
              <span class="value avg">{{ efficiencyData?.avg_efficiency || 0 }}</span>
            </div>
            <div class="stat-item">
              <span class="label">最低效率</span>
              <span class="value low">{{ efficiencyData?.low_efficiency || 0 }}</span>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- Order-Chute Binding Status - 订单格口绑定状态 -->
    <el-row :gutter="20" class="section-row">
      <el-col :span="12">
        <el-card>
          <template #header>
            <div class="card-header-title">
              <span>订单-格口绑定</span>
              <el-button type="primary" size="small" @click="viewAllBindings">查看全部</el-button>
            </div>
          </template>
          <el-table :data="orderChuteBindings" v-loading="bindingLoading" style="width: 100%" max-height="300">
            <el-table-column prop="order_id" label="订单号" width="120" show-overflow-tooltip />
            <el-table-column prop="chute_id" label="格口号" width="100" />
            <el-table-column prop="platform_id" label="平台" width="100" />
            <el-table-column prop="status" label="状态" width="80">
              <template #default="scope">
                <el-tag :type="scope.row.status === 'ACTIVE' ? 'success' : 'info'" size="small">
                  {{ scope.row.status === 'ACTIVE' ? '绑定中' : '已释放' }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="created_at" label="绑定时间">
              <template #default="scope">
                {{ formatTime(scope.row.created_at) }}
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-col>
      <el-col :span="12">
        <el-card>
          <template #header>
            <div class="card-header-title">
              <span>任务分配日志</span>
              <el-button type="primary" size="small" @click="clearTaskLogs">清空</el-button>
            </div>
          </template>
          <div class="task-log-container">
            <div v-for="(log, index) in taskLogs" :key="index" class="task-log-item" :class="log.type">
              <span class="time">{{ log.time }}</span>
              <span class="message">{{ log.message }}</span>
              <el-tag :type="log.status === 'success' ? 'success' : 'danger'" size="small">
                {{ log.status === 'success' ? '成功' : '失败' }}
              </el-tag>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <!-- Wave List -->
    <el-row class="table-row">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="card-header-title">
              <span>波次列表</span>
              <el-button type="primary" size="small" @click="refreshWaves">
                <el-icon><Refresh /></el-icon>刷新
              </el-button>
            </div>
          </template>
          <el-table :data="waveList" v-loading="loading" style="width: 100%">
            <el-table-column prop="wave_id" label="波次ID" width="180" />
            <el-table-column prop="platform_id" label="平台" width="120" />
            <el-table-column prop="create_time" label="创建时间" width="180">
              <template #default="scope">
                {{ formatTime(scope.row.create_time) }}
              </template>
            </el-table-column>
            <el-table-column prop="order_qty" label="订单数" width="100" />
            <el-table-column prop="all_qty" label="总数量" width="100" />
            <el-table-column prop="speed" label="速度" width="100">
              <template #default="scope">
                {{ scope.row.speed?.toFixed(1) || 0 }}件/分
              </template>
            </el-table-column>
            <el-table-column prop="wave_status_name" label="状态" width="120">
              <template #default="scope">
                <el-tag :type="getStatusType(scope.row.wave_status)">
                  {{ scope.row.wave_status_name }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="rate" label="完成率" width="150">
              <template #default="scope">
                <el-progress :percentage="scope.row.rate || 0" />
              </template>
            </el-table-column>
            <el-table-column label="操作" width="200">
              <template #default="scope">
                <el-button 
                  v-if="scope.row.wave_status === 'UN_STARTED'"
                  type="primary" 
                  size="small"
                  @click="startWave(scope.row.wave_id)"
                >
                  启动
                </el-button>
                <el-button 
                  v-if="scope.row.wave_status === 'STARTED'"
                  type="danger" 
                  size="small"
                  @click="closeWave(scope.row.wave_id)"
                >
                  关闭
                </el-button>
                <el-button size="small" @click="viewDetail(scope.row)">详情</el-button>
              </template>
            </el-table-column>
          </el-table>
          <el-pagination
            v-model:current-page="pageNum"
            v-model:page-size="pageSize"
            :total="total"
            layout="total, sizes, prev, pager, next"
            @size-change="handleSizeChange"
            @current-change="handleCurrentChange"
          />
        </el-card>
      </el-col>
    </el-row>

    <!-- Station List with SSE Status - 工作站列表含SSE状态 -->
    <el-row class="table-row">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="card-header-title">
              <span>工作站状态</span>
              <el-tag type="info">SSE在线: {{ onlineStations }}/{{ stationList.length }}</el-tag>
            </div>
          </template>
          <el-table :data="stationList" v-loading="stationLoading" style="width: 100%">
            <el-table-column prop="station_id" label="工作站ID" width="150" />
            <el-table-column prop="platform_id" label="平台" width="120" />
            <el-table-column prop="station_status_name" label="状态" width="100">
              <template #default="scope">
                <el-tag :type="scope.row.station_status === 'enable' ? 'success' : 'info'">
                  {{ scope.row.station_status_name }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column label="SSE连接" width="100">
              <template #default="scope">
                <el-tag :type="scope.row.sse_connected ? 'success' : 'info'" size="small">
                  {{ scope.row.sse_connected ? '在线' : '离线' }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="current_agv_id" label="当前车辆" width="120">
              <template #default="scope">
                <span v-if="scope.row.current_agv_id">
                  <el-icon><Van /></el-icon> {{ scope.row.current_agv_id }}
                </span>
                <span v-else>-</span>
              </template>
            </el-table-column>
            <el-table-column prop="current_wave_id" label="当前波次" width="150" show-overflow-tooltip />
            <el-table-column prop="sort_qty" label="分拣数量" width="100" />
            <el-table-column prop="sort_efficiency" label="分拣效率" width="120" />
            <el-table-column prop="efficiency" label="效率" width="100">
              <template #default="scope">
                {{ scope.row.efficiency?.toFixed(1) || 0 }}%
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed } from 'vue'
import { ElMessage } from 'element-plus'
import { Refresh, Van } from '@element-plus/icons-vue'
import * as echarts from 'echarts'
import axios from 'axios'

const API_BASE = '/api/v1'

const systemStatus = ref(null)
const efficiencyData = ref(null)
const waveList = ref([])
const stationList = ref([])
const platforms = ref([])
const orderChuteBindings = ref([])
const taskLogs = ref([])
const loading = ref(false)
const stationLoading = ref(false)
const bindingLoading = ref(false)
const pageNum = ref(1)
const pageSize = ref(10)
const total = ref(0)

// RCS & SSE status
const rcsConnected = ref(true)
const rcsLatency = ref(15)
const sseConnections = ref(0)
const stationConnections = ref(0)
const rcsConnections = ref(0)

// Chute occupancy
const activeChutes = ref(0)
const totalChutes = ref(0)
const boundOrders = ref(0)

const efficiencyChart = ref(null)
let chartInstance = null
let refreshTimer = null

// Computed
const chuteOccupancyRate = computed(() => {
  if (totalChutes.value === 0) return 0
  return Math.round((activeChutes.value / totalChutes.value) * 100)
})

const onlineStations = computed(() => {
  return stationList.value.filter(s => s.sse_connected).length
})

const getOccupancyColor = computed(() => {
  return (percentage) => {
    if (percentage < 50) return '#67C23A'
    if (percentage < 80) return '#E6A23C'
    return '#F56C6C'
  }
})

// Fetch home page data
const fetchHomePage = async () => {
  try {
    const response = await axios.get(`${API_BASE}/waveInfo/home/page`)
    if (response.data.code === 0) {
      const data = response.data.data
      systemStatus.value = data.system_status_vo
      efficiencyData.value = data.sort_efficiency_vo
      initChart()
    }
  } catch (error) {
    ElMessage.error('获取首页数据失败')
  }
}

// Fetch platforms
const fetchPlatforms = async () => {
  try {
    const response = await axios.get(`${API_BASE}/platforms`)
    if (response.data.code === 0) {
      platforms.value = response.data.data || []
    }
  } catch (error) {
    console.error('获取平台数据失败:', error)
  }
}

// Fetch wave list
const fetchWaveList = async () => {
  loading.value = true
  try {
    const response = await axios.post(`${API_BASE}/waveInfo/home/page/wave`, {
      pageNum: pageNum.value - 1,
      pageSize: pageSize.value
    })
    if (response.data.code === 0) {
      waveList.value = response.data.data.data
      total.value = response.data.data.total
    }
  } catch (error) {
    ElMessage.error('获取波次列表失败')
  } finally {
    loading.value = false
  }
}

// Fetch station list
const fetchStationList = async () => {
  stationLoading.value = true
  try {
    const response = await axios.post(`${API_BASE}/waveInfo/home/page/station`, {
      pageNum: 0,
      pageSize: 20
    })
    if (response.data.code === 0) {
      stationList.value = response.data.data.data
    }
  } catch (error) {
    ElMessage.error('获取工作站列表失败')
  } finally {
    stationLoading.value = false
  }
}

// Fetch order-chute bindings
const fetchOrderChuteBindings = async () => {
  bindingLoading.value = true
  try {
    const response = await axios.get(`${API_BASE}/order-chute-bindings?limit=10`)
    if (response.data.code === 0) {
      orderChuteBindings.value = response.data.data || []
      activeChutes.value = orderChuteBindings.value.filter(b => b.status === 'ACTIVE').length
    }
  } catch (error) {
    console.error('获取绑定数据失败:', error)
  } finally {
    bindingLoading.value = false
  }
}

// Check RCS connection
const checkRcsConnection = async () => {
  try {
    const start = Date.now()
    const response = await axios.get(`${API_BASE}/rcs/health`, { timeout: 5000 })
    rcsLatency.value = Date.now() - start
    rcsConnected.value = response.data.code === 0
  } catch (error) {
    rcsConnected.value = false
    rcsLatency.value = 999
  }
}

// Fetch SSE status
const fetchSseStatus = async () => {
  try {
    const response = await axios.get(`${API_BASE}/sse/status`)
    if (response.data.code === 0) {
      const data = response.data.data
      sseConnections.value = data.total_connections || 0
      stationConnections.value = data.station_connections || 0
      rcsConnections.value = data.rcs_connections || 0
    }
  } catch (error) {
    console.error('获取SSE状态失败:', error)
  }
}

// Fetch chute stats
const fetchChuteStats = async () => {
  try {
    const response = await axios.get(`${API_BASE}/chutes/stats`)
    if (response.data.code === 0) {
      const data = response.data.data
      totalChutes.value = data.total || 0
      boundOrders.value = data.bound_orders || 0
    }
  } catch (error) {
    console.error('获取格口统计失败:', error)
  }
}

// Initialize efficiency chart
const initChart = () => {
  if (!efficiencyChart.value || !efficiencyData.value) return
  
  if (chartInstance) {
    chartInstance.dispose()
  }
  
  chartInstance = echarts.init(efficiencyChart.value)
  const option = {
    tooltip: { trigger: 'axis' },
    xAxis: {
      type: 'category',
      data: efficiencyData.value.times || []
    },
    yAxis: {
      type: 'value',
      name: '效率(件/小时)'
    },
    series: [{
      data: efficiencyData.value.efficiencies || [],
      type: 'line',
      smooth: true,
      areaStyle: {
        color: {
          type: 'linear',
          x: 0, y: 0, x2: 0, y2: 1,
          colorStops: [
            { offset: 0, color: 'rgba(64, 158, 255, 0.3)' },
            { offset: 1, color: 'rgba(64, 158, 255, 0.05)' }
          ]
        }
      },
      lineStyle: { color: '#409EFF', width: 3 },
      itemStyle: { color: '#409EFF' }
    }]
  }
  chartInstance.setOption(option)
}

// Start wave
const startWave = async (waveId) => {
  try {
    const response = await axios.get(`${API_BASE}/waves/start/${waveId}`)
    if (response.data.code === 0) {
      ElMessage.success('波次启动成功')
      addTaskLog(`启动波次 ${waveId}`, 'success')
      fetchWaveList()
    }
  } catch (error) {
    ElMessage.error('启动波次失败')
    addTaskLog(`启动波次 ${waveId} 失败`, 'error')
  }
}

// Close wave
const closeWave = async (waveId) => {
  try {
    const response = await axios.get(`${API_BASE}/waves/close/${waveId}`)
    if (response.data.code === 0) {
      ElMessage.success('波次关闭成功')
      addTaskLog(`关闭波次 ${waveId}`, 'success')
      fetchWaveList()
    }
  } catch (error) {
    ElMessage.error('关闭波次失败')
    addTaskLog(`关闭波次 ${waveId} 失败`, 'error')
  }
}

// View detail
const viewDetail = (row) => {
  ElMessage.info(`查看波次 ${row.wave_id} 详情`)
}

// Refresh data
const refreshWaves = () => {
  fetchWaveList()
  fetchHomePage()
}

const refreshPlatforms = () => {
  fetchPlatforms()
}

// View all bindings
const viewAllBindings = () => {
  ElMessage.info('查看全部绑定')
}

// Clear task logs
const clearTaskLogs = () => {
  taskLogs.value = []
}

// Add task log
const addTaskLog = (message, status) => {
  taskLogs.value.unshift({
    time: new Date().toLocaleTimeString(),
    message,
    status,
    type: status === 'success' ? 'success' : 'error'
  })
  if (taskLogs.value.length > 20) {
    taskLogs.value.pop()
  }
}

// Handle size change
const handleSizeChange = (val) => {
  pageSize.value = val
  fetchWaveList()
}

// Handle current change
const handleCurrentChange = (val) => {
  pageNum.value = val
  fetchWaveList()
}

// Format time
const formatTime = (time) => {
  if (!time) return '-'
  return new Date(time).toLocaleString()
}

// Get status type
const getStatusType = (status) => {
  const types = {
    'UN_STARTED': 'info',
    'STARTED': 'warning',
    'CLOSED': 'success',
    'AUTO_COMPLETE': 'success'
  }
  return types[status] || 'info'
}

// Auto refresh
const startAutoRefresh = () => {
  refreshTimer = setInterval(() => {
    checkRcsConnection()
    fetchSseStatus()
    fetchChuteStats()
    fetchOrderChuteBindings()
  }, 5000)
}

onMounted(() => {
  fetchHomePage()
  fetchWaveList()
  fetchStationList()
  fetchPlatforms()
  fetchOrderChuteBindings()
  checkRcsConnection()
  fetchSseStatus()
  fetchChuteStats()
  startAutoRefresh()
  window.addEventListener('resize', () => chartInstance?.resize())
})

onUnmounted(() => {
  window.removeEventListener('resize', () => chartInstance?.resize())
  chartInstance?.dispose()
  if (refreshTimer) clearInterval(refreshTimer)
})
</script>

<style scoped>
.dashboard {
  padding: 20px;
}

.status-row {
  margin-bottom: 20px;
}

.status-card {
  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    color: #666;
    font-size: 14px;
    margin-bottom: 10px;
  }
  
  .card-value {
    font-size: 28px;
    font-weight: bold;
    color: #303133;
    margin: 10px 0;
  }
  
  .card-label {
    font-size: 12px;
    color: #909399;
  }
  
  &.warning {
    border: 1px solid #F56C6C;
    .card-value { color: #F56C6C; }
  }
}

.section-row {
  margin-bottom: 20px;
}

.platform-card {
  background: #f5f7fa;
  border-radius: 8px;
  padding: 15px;
  margin-bottom: 10px;
  
  .platform-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
    
    .platform-name {
      font-weight: bold;
      font-size: 16px;
      color: #303133;
    }
  }
  
  .platform-stats {
    display: flex;
    gap: 20px;
    margin-bottom: 12px;
    
    .stat {
      display: flex;
      flex-direction: column;
      
      .label {
        font-size: 12px;
        color: #909399;
      }
      
      .value {
        font-size: 18px;
        font-weight: bold;
        color: #409EFF;
      }
    }
  }
  
  .platform-progress {
    .label {
      font-size: 12px;
      color: #909399;
      margin-bottom: 4px;
      display: block;
    }
  }
}

.chart-row {
  margin-bottom: 20px;
}

.chart-container {
  height: 300px;
}

.card-header-title {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.avg-efficiency {
  font-size: 14px;
  color: #409EFF;
}

.efficiency-stats {
  .stat-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 15px 0;
    border-bottom: 1px solid #EBEEF5;
    
    &:last-child { border-bottom: none; }
    
    .label { color: #606266; }
    
    .value {
      font-size: 20px;
      font-weight: bold;
      
      &.high { color: #67C23A; }
      &.avg { color: #409EFF; }
      &.low { color: #F56C6C; }
    }
  }
}

.task-log-container {
  max-height: 300px;
  overflow-y: auto;
  
  .task-log-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 0;
    border-bottom: 1px solid #EBEEF5;
    font-size: 13px;
    
    .time {
      color: #909399;
      font-size: 12px;
      min-width: 70px;
    }
    
    .message {
      flex: 1;
      color: #606266;
    }
    
    &.success .message { color: #67C23A; }
    &.error .message { color: #F56C6C; }
  }
}

.table-row {
  margin-bottom: 20px;
}

h1 {
  margin-bottom: 20px;
  color: #303133;
}
</style>
