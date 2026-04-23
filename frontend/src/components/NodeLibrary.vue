<template>
  <div class="node-library">
    <div class="library-header">
      <h4>节点库</h4>
      <el-input v-model="searchText" placeholder="搜索节点" size="small" clearable>
        <template #prefix>
          <el-icon><Search /></el-icon>
        </template>
      </el-input>
    </div>

    <div class="library-content">
      <el-collapse v-model="activeCategories">
        <!-- 业务流程节点 -->
        <el-collapse-item title="业务流程 (Business)" name="business">
          <div class="category-desc">SES核心业务流程节点</div>
          <div class="node-list">
            <div
              v-for="node in businessNodes"
              :key="node.node_id"
              class="node-item node-business"
              draggable="true"
              @dragstart="(e) => onDragStart(e, node)"
            >
              <div class="node-icon" :style="{ backgroundColor: node.color }">
                <el-icon :size="16" color="#fff">
                  <component :is="getIcon(node.icon)" />
                </el-icon>
              </div>
              <div class="node-info">
                <span class="node-label">{{ node.name }}</span>
                <span class="node-desc">{{ node.description }}</span>
              </div>
            </div>
          </div>
        </el-collapse-item>

        <!-- RCS 对接节点 -->
        <el-collapse-item title="RCS 对接 (RCS Integration)" name="rcs">
          <div class="category-desc">机器人调度系统对接节点</div>
          <div class="node-list">
            <div
              v-for="node in rcsNodes"
              :key="node.node_id"
              class="node-item node-rcs"
              draggable="true"
              @dragstart="(e) => onDragStart(e, node)"
            >
              <div class="node-icon" :style="{ backgroundColor: node.color }">
                <el-icon :size="16" color="#fff">
                  <component :is="getIcon(node.icon)" />
                </el-icon>
              </div>
              <div class="node-info">
                <span class="node-label">{{ node.name }}</span>
                <span class="node-desc">{{ node.description }}</span>
              </div>
            </div>
          </div>
        </el-collapse-item>

        <!-- 工作站节点 -->
        <el-collapse-item title="工作站 (Station)" name="station">
          <div class="category-desc">供包工作站业务节点</div>
          <div class="node-list">
            <div
              v-for="node in stationNodes"
              :key="node.node_id"
              class="node-item node-station"
              draggable="true"
              @dragstart="(e) => onDragStart(e, node)"
            >
              <div class="node-icon" :style="{ backgroundColor: node.color }">
                <el-icon :size="16" color="#fff">
                  <component :is="getIcon(node.icon)" />
                </el-icon>
              </div>
              <div class="node-info">
                <span class="node-label">{{ node.name }}</span>
                <span class="node-desc">{{ node.description }}</span>
              </div>
            </div>
          </div>
        </el-collapse-item>

        <!-- 任务分配节点 -->
        <el-collapse-item title="任务分配 (Task Dispatch)" name="task">
          <div class="category-desc">目的地分配与任务下发节点</div>
          <div class="node-list">
            <div
              v-for="node in taskNodes"
              :key="node.node_id"
              class="node-item node-task"
              draggable="true"
              @dragstart="(e) => onDragStart(e, node)"
            >
              <div class="node-icon" :style="{ backgroundColor: node.color }">
                <el-icon :size="16" color="#fff">
                  <component :is="getIcon(node.icon)" />
                </el-icon>
              </div>
              <div class="node-info">
                <span class="node-label">{{ node.name }}</span>
                <span class="node-desc">{{ node.description }}</span>
              </div>
            </div>
          </div>
        </el-collapse-item>

        <!-- 设备控制节点 -->
        <el-collapse-item title="设备控制 (Device)" name="device">
          <div class="category-desc">硬件设备控制节点</div>
          <div class="node-list">
            <div
              v-for="node in deviceNodes"
              :key="node.node_id"
              class="node-item node-device"
              draggable="true"
              @dragstart="(e) => onDragStart(e, node)"
            >
              <div class="node-icon" :style="{ backgroundColor: node.color }">
                <el-icon :size="16" color="#fff">
                  <component :is="getIcon(node.icon)" />
                </el-icon>
              </div>
              <div class="node-info">
                <span class="node-label">{{ node.name }}</span>
                <span class="node-desc">{{ node.description }}</span>
              </div>
            </div>
          </div>
        </el-collapse-item>

        <!-- 逻辑控制节点 -->
        <el-collapse-item title="逻辑控制 (Logic)" name="logic">
          <div class="category-desc">流程控制与条件判断节点</div>
          <div class="node-list">
            <div
              v-for="node in logicNodes"
              :key="node.node_id"
              class="node-item node-logic"
              draggable="true"
              @dragstart="(e) => onDragStart(e, node)"
            >
              <div class="node-icon" :style="{ backgroundColor: node.color }">
                <el-icon :size="16" color="#fff">
                  <component :is="getIcon(node.icon)" />
                </el-icon>
              </div>
              <div class="node-info">
                <span class="node-label">{{ node.name }}</span>
                <span class="node-desc">{{ node.description }}</span>
              </div>
            </div>
          </div>
        </el-collapse-item>

        <!-- 系统节点 -->
        <el-collapse-item title="系统 (System)" name="system">
          <div class="category-desc">系统级通用节点</div>
          <div class="node-list">
            <div
              v-for="node in systemNodes"
              :key="node.node_id"
              class="node-item node-system"
              draggable="true"
              @dragstart="(e) => onDragStart(e, node)"
            >
              <div class="node-icon" :style="{ backgroundColor: node.color }">
                <el-icon :size="16" color="#fff">
                  <component :is="getIcon(node.icon)" />
                </el-icon>
              </div>
              <div class="node-info">
                <span class="node-label">{{ node.name }}</span>
                <span class="node-desc">{{ node.description }}</span>
              </div>
            </div>
          </div>
        </el-collapse-item>
      </el-collapse>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'

interface NodeDefinition {
  node_id: string
  name: string
  description?: string
  kind: string
  config_schema?: any
  default_config?: any
  icon?: string
  color?: string
  category: string
}

const searchText = ref('')
const activeCategories = ref(['business', 'rcs', 'station', 'task', 'device', 'logic', 'system'])

// 业务流程节点
const businessNodes = ref<NodeDefinition[]>([
  {
    node_id: 'wave_create',
    name: '创建波次',
    description: '创建新的分拣波次，绑定平台',
    kind: 'business',
    category: 'business',
    icon: 'FolderAdd',
    color: '#409EFF'
  },
  {
    node_id: 'wave_start',
    name: '启动波次',
    description: '启动波次，开始分拣作业',
    kind: 'business',
    category: 'business',
    icon: 'VideoPlay',
    color: '#67C23A'
  },
  {
    node_id: 'wave_close',
    name: '关闭波次',
    description: '关闭波次，结束分拣作业',
    kind: 'business',
    category: 'business',
    icon: 'CircleClose',
    color: '#F56C6C'
  },
  {
    node_id: 'order_assign',
    name: '分配订单',
    description: '将订单分配到工作站',
    kind: 'business',
    category: 'business',
    icon: 'Connection',
    color: '#E6A23C'
  },
  {
    node_id: 'order_close',
    name: '关闭订单',
    description: '完成订单并释放资源',
    kind: 'business',
    category: 'business',
    icon: 'Finished',
    color: '#909399'
  }
])

// RCS 对接节点
const rcsNodes = ref<NodeDefinition[]>([
  {
    node_id: 'rcs_dispatch',
    name: 'RCS任务下发',
    description: '向RCS下发机器人任务',
    kind: 'rcs',
    category: 'rcs',
    icon: 'Promotion',
    color: '#9254DE'
  },
  {
    node_id: 'rcs_arrived',
    name: '车辆到达',
    description: '处理RCS上报的车辆到达',
    kind: 'rcs',
    category: 'rcs',
    icon: 'Location',
    color: '#13C2C2'
  },
  {
    node_id: 'rcs_request_flip',
    name: '请求卸货',
    description: '处理RCS卸货请求',
    kind: 'rcs',
    category: 'rcs',
    icon: 'Bottom',
    color: '#FA8C16'
  },
  {
    node_id: 'rcs_task_cancel',
    name: '取消任务',
    description: '向RCS取消机器人任务',
    kind: 'rcs',
    category: 'rcs',
    icon: 'CloseBold',
    color: '#FF4D4F'
  },
  {
    node_id: 'rcs_switch_grid',
    name: '切换格口',
    description: '批量修改格口状态',
    kind: 'rcs',
    category: 'rcs',
    icon: 'Switch',
    color: '#1890FF'
  }
])

// 工作站节点
const stationNodes = ref<NodeDefinition[]>([
  {
    node_id: 'station_get_task',
    name: '获取任务',
    description: '工作站获取当前任务',
    kind: 'station',
    category: 'station',
    icon: 'List',
    color: '#52C41A'
  },
  {
    node_id: 'station_scan',
    name: '扫码',
    description: '处理工作站扫码事件',
    kind: 'station',
    category: 'station',
    icon: 'FullScreen',
    color: '#722ED1'
  },
  {
    node_id: 'station_online',
    name: '工作站上线',
    description: '工作站上线连接',
    kind: 'station',
    category: 'station',
    icon: 'CircleCheck',
    color: '#67C23A'
  },
  {
    node_id: 'station_offline',
    name: '工作站下线',
    description: '工作站下线断开',
    kind: 'station',
    category: 'station',
    icon: 'CircleClose',
    color: '#909399'
  },
  {
    node_id: 'chute_operation',
    name: '格口操作',
    description: '打开/关闭格口',
    kind: 'station',
    category: 'station',
    icon: 'Open',
    color: '#FAAD14'
  }
])

// 任务分配节点
const taskNodes = ref<NodeDefinition[]>([
  {
    node_id: 'task_dispatch',
    name: '任务分配',
    description: '为订单分配空闲目的地格口',
    kind: 'task',
    category: 'task',
    icon: 'MapLocation',
    color: '#EB2F96'
  },
  {
    node_id: 'task_deduct',
    name: '数量扣减',
    description: '扣减订单SKU完成数量',
    kind: 'task',
    category: 'task',
    icon: 'Minus',
    color: '#FF7A45'
  },
  {
    node_id: 'destination_bind',
    name: '目的地绑定',
    description: '绑定订单与目的地格口',
    kind: 'task',
    category: 'task',
    icon: 'Link',
    color: '#36CFC9'
  },
  {
    node_id: 'destination_release',
    name: '目的地释放',
    description: '释放订单绑定的格口',
    kind: 'task',
    category: 'task',
    icon: 'Unlock',
    color: '#73D13D'
  }
])

// 设备控制节点
const deviceNodes = ref<NodeDefinition[]>([
  {
    node_id: 'device_command',
    name: '设备命令',
    description: '发送设备控制命令',
    kind: 'device',
    category: 'device',
    icon: 'Cpu',
    color: '#2F54EB'
  },
  {
    node_id: 'wall_online',
    name: '播种墙上线',
    description: '播种墙上线操作',
    kind: 'device',
    category: 'device',
    icon: 'Monitor',
    color: '#597EF7'
  },
  {
    node_id: 'wall_offline',
    name: '播种墙下线',
    description: '播种墙下线操作',
    kind: 'device',
    category: 'device',
    icon: 'Monitor',
    color: '#B37FEB'
  },
  {
    node_id: 'pack_operation',
    name: '封包操作',
    description: '执行封包动作',
    kind: 'device',
    category: 'device',
    icon: 'Box',
    color: '#FFC53D'
  },
  {
    node_id: 'distribute_item',
    name: '人工播种',
    description: '人工播种操作',
    kind: 'device',
    category: 'device',
    icon: 'Handbag',
    color: '#FF9C6E'
  }
])

// 逻辑控制节点
const logicNodes = ref<NodeDefinition[]>([
  {
    node_id: 'condition',
    name: '条件判断',
    description: '根据条件分支执行',
    kind: 'logic',
    category: 'logic',
    icon: 'Share',
    color: '#73D13D'
  },
  {
    node_id: 'parallel',
    name: '并行执行',
    description: '并行执行多个分支',
    kind: 'logic',
    category: 'logic',
    icon: 'ForkSpoon',
    color: '#40A9FF'
  },
  {
    node_id: 'delay',
    name: '延迟',
    description: '延迟指定时间',
    kind: 'logic',
    category: 'logic',
    icon: 'Timer',
    color: '#FFC53D'
  },
  {
    node_id: 'retry',
    name: '重试',
    description: '失败时重试',
    kind: 'logic',
    category: 'logic',
    icon: 'RefreshRight',
    color: '#FFA940'
  }
])

// 系统节点
const systemNodes = ref<NodeDefinition[]>([
  {
    node_id: 'start',
    name: '开始',
    description: '工作流起始节点',
    kind: 'system',
    category: 'system',
    icon: 'CircleCheck',
    color: '#67C23A'
  },
  {
    node_id: 'end',
    name: '结束',
    description: '工作流结束节点',
    kind: 'system',
    category: 'system',
    icon: 'CircleCloseFilled',
    color: '#909399'
  },
  {
    node_id: 'http_request',
    name: 'HTTP请求',
    description: '发送HTTP请求',
    kind: 'system',
    category: 'system',
    icon: 'Promotion',
    color: '#1890FF'
  },
  {
    node_id: 'log',
    name: '日志',
    description: '记录日志',
    kind: 'system',
    category: 'system',
    icon: 'Document',
    color: '#8C8C8C'
  },
  {
    node_id: 'variable',
    name: '变量',
    description: '设置变量',
    kind: 'system',
    category: 'system',
    icon: 'Edit',
    color: '#595959'
  }
])

// 搜索过滤
const filterNodes = (nodes: NodeDefinition[]) => {
  if (!searchText.value) return nodes
  const search = searchText.value.toLowerCase()
  return nodes.filter(node => 
    node.name.toLowerCase().includes(search) ||
    node.node_id.toLowerCase().includes(search) ||
    (node.description && node.description.toLowerCase().includes(search))
  )
}

// 获取图标组件
const getIcon = (iconName?: string) => {
  if (!iconName) return 'CircleCheck'
  return (ElementPlusIconsVue as any)[iconName] || 'CircleCheck'
}

// 拖拽开始
const onDragStart = (event: DragEvent, node: NodeDefinition) => {
  if (event.dataTransfer) {
    event.dataTransfer.setData('application/vueflow', JSON.stringify(node))
    event.dataTransfer.effectAllowed = 'move'
  }
}
</script>

<style scoped>
.node-library {
  width: 280px;
  background: #fff;
  border-right: 1px solid #dcdfe6;
  display: flex;
  flex-direction: column;
}

.library-header {
  padding: 16px;
  border-bottom: 1px solid #ebeef5;
}

.library-header h4 {
  margin: 0 0 12px 0;
  font-size: 16px;
  color: #303133;
}

.library-content {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.category-desc {
  font-size: 11px;
  color: #909399;
  padding: 0 8px 8px;
  border-bottom: 1px solid #f0f0f0;
  margin-bottom: 8px;
}

.node-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.node-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border-radius: 6px;
  cursor: move;
  transition: all 0.2s;
  border: 1px solid #ebeef5;
  background: #fff;
}

.node-item:hover {
  background: #f5f7fa;
  border-color: #2EC6D6;
  box-shadow: 0 2px 8px rgba(46, 198, 214, 0.15);
  transform: translateX(4px);
}

.node-icon {
  width: 32px;
  height: 32px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.node-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  flex: 1;
  min-width: 0;
}

.node-label {
  font-size: 13px;
  color: #303133;
  font-weight: 500;
}

.node-desc {
  font-size: 11px;
  color: #909399;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* 节点类型颜色标记 */
.node-business { border-left: 3px solid #409EFF; }
.node-rcs { border-left: 3px solid #9254DE; }
.node-station { border-left: 3px solid #52C41A; }
.node-task { border-left: 3px solid #EB2F96; }
.node-device { border-left: 3px solid #2F54EB; }
.node-logic { border-left: 3px solid #73D13D; }
.node-system { border-left: 3px solid #909399; }
</style>
