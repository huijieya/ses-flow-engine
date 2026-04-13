<template>
  <div class="dashboard">
    <h1>SES 系统概览</h1>
    
    <!-- System Status Cards -->
    <el-row :gutter="20" class="status-row">
      <el-col :span="6">
        <el-card class="status-card">
          <div class="card-header">
            <span>今日波次</span>
            <el-tag type="success">+5%</el-tag>
          </div>
          <div class="card-value">{{ systemStatus?.twave_qty || 0 }}</div>
          <div class="card-label">较昨日 +{{ systemStatus?.twave_qty_compare_yest || 0 }}</div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="status-card">
          <div class="card-header">
            <span>已完成波次</span>
            <el-tag type="primary">进行中</el-tag>
          </div>
          <div class="card-value">{{ systemStatus?.twave_completed || 0 }}</div>
          <div class="card-label">较昨日 +{{ systemStatus?.twave_completed_compare_yest || 0 }}</div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="status-card">
          <div class="card-header">
            <span>分拣数量</span>
            <el-tag type="warning">实时</el-tag>
          </div>
          <div class="card-value">{{ systemStatus?.tsorting_qty || 0 }}</div>
          <div class="card-label">较昨日 +{{ systemStatus?.tsorting_qty_compare_yest || 0 }}</div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="status-card">
          <div class="card-header">
            <span>封包数量</span>
            <el-tag>累计</el-tag>
          </div>
          <div class="card-value">{{ systemStatus?.tpack_qty || 0 }}</div>
          <div class="card-label">较昨日 +{{ systemStatus?.tpack_qty_compare_yest || 0 }}</div>
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

    <!-- Wave List -->
    <el-row class="table-row">
      <el-col :span="24">
        <el-card>
          <template #header>
            <div class="card-header-title">
              <span>波次列表</span>
              <el-button type="primary" size="small" @click="refreshWaves">刷新</el-button>
            </div>
          </template>
          <el-table :data="waveList" v-loading="loading" style="width: 100%">
            <el-table-column prop="wave_id" label="波次ID" width="180" />
            <el-table-column prop="create_time" label="创建时间" width="180">
              <template #default="scope">
                {{ formatTime(scope.row.create_time) }}
              </template>
            </el-table-column>
            <el-table-column prop="order_qty" label="订单数" width="100" />
            <el-table-column prop="all_qty" label="总数量" width="100" />
            <el-table-column prop="speed" label="速度" width="100">
              <template #default="scope">
                {{ scope.row.speed.toFixed(1) }}件/分
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
                <el-progress :percentage="scope.row.rate" />
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

    <!-- Station List -->
    <el-row class="table-row">
      <el-col :span="24">
        <el-card>
          <template #header>
            <span>工作站状态</span>
          </template>
          <el-table :data="stationList" v-loading="stationLoading" style="width: 100%">
            <el-table-column prop="station_id" label="工作站ID" width="180" />
            <el-table-column prop="sort_qty" label="分拣数量" width="120" />
            <el-table-column prop="efficiency" label="效率" width="120">
              <template #default="scope">
                {{ scope.row.efficiency.toFixed(1) }}%
              </template>
            </el-table-column>
            <el-table-column prop="sort_efficiency" label="分拣效率" width="120" />
            <el-table-column prop="station_status_name" label="状态" width="120">
              <template #default="scope">
                <el-tag :type="scope.row.station_status === 'enable' ? 'success' : 'info'">
                  {{ scope.row.station_status_name }}
                </el-tag>
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { ElMessage } from 'element-plus'
import * as echarts from 'echarts'
import axios from 'axios'

const API_BASE = '/api/v1'

const systemStatus = ref(null)
const efficiencyData = ref(null)
const waveList = ref([])
const stationList = ref([])
const loading = ref(false)
const stationLoading = ref(false)
const pageNum = ref(1)
const pageSize = ref(10)
const total = ref(0)
const efficiencyChart = ref(null)
let chartInstance = null

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

// Initialize efficiency chart
const initChart = () => {
  if (!efficiencyChart.value || !efficiencyData.value) return
  
  if (chartInstance) {
    chartInstance.dispose()
  }
  
  chartInstance = echarts.init(efficiencyChart.value)
  const option = {
    tooltip: {
      trigger: 'axis'
    },
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
      lineStyle: {
        color: '#409EFF',
        width: 3
      },
      itemStyle: {
        color: '#409EFF'
      }
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
      fetchWaveList()
    }
  } catch (error) {
    ElMessage.error('启动波次失败')
  }
}

// Close wave
const closeWave = async (waveId) => {
  try {
    const response = await axios.get(`${API_BASE}/waves/close/${waveId}`)
    if (response.data.code === 0) {
      ElMessage.success('波次关闭成功')
      fetchWaveList()
    }
  } catch (error) {
    ElMessage.error('关闭波次失败')
  }
}

// View detail
const viewDetail = (row) => {
  ElMessage.info(`查看波次 ${row.wave_id} 详情`)
}

// Refresh waves
const refreshWaves = () => {
  fetchWaveList()
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

onMounted(() => {
  fetchHomePage()
  fetchWaveList()
  fetchStationList()
  window.addEventListener('resize', () => chartInstance?.resize())
})

onUnmounted(() => {
  window.removeEventListener('resize', () => chartInstance?.resize())
  chartInstance?.dispose()
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
    font-size: 32px;
    font-weight: bold;
    color: #303133;
    margin: 10px 0;
  }
  
  .card-label {
    font-size: 12px;
    color: #909399;
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
    
    &:last-child {
      border-bottom: none;
    }
    
    .label {
      color: #606266;
    }
    
    .value {
      font-size: 20px;
      font-weight: bold;
      
      &.high {
        color: #67C23A;
      }
      
      &.avg {
        color: #409EFF;
      }
      
      &.low {
        color: #F56C6C;
      }
    }
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
