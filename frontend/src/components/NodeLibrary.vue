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
        <el-collapse-item v-for="category in filteredCategories" :key="category.name" :title="category.name" :name="category.name">
          <div class="node-list">
            <div
              v-for="node in category.nodes"
              :key="node.type"
              class="node-item"
              :class="[`node-${node.kind}`]"
              draggable="true"
              @dragstart="(e) => onDragStart(e, node)"
            >
              <el-icon :size="20">
                <component :is="node.icon" />
              </el-icon>
              <span class="node-label">{{ node.label }}</span>
            </div>
          </div>
        </el-collapse-item>
      </el-collapse>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

const props = defineProps<{
  onDragStart: (event: DragEvent, nodeType: string) => void
}>()

const searchText = ref('')
const activeCategories = ref(['设备节点', '逻辑节点', '数据节点', '系统节点'])

const categories = ref([
  {
    name: '设备节点',
    nodes: [
      { type: 'chute_operate', label: '格口操作', kind: 'device', icon: 'Box' },
      { type: 'chute_open', label: '打开格口', kind: 'device', icon: 'Open' },
      { type: 'chute_close', label: '关闭格口', kind: 'device', icon: 'TurnOff' },
      { type: 'device_command', label: '设备命令', kind: 'device', icon: 'SetUp' },
      { type: 'device_set_light', label: '设置灯光', kind: 'device', icon: 'Orange' },
      { type: 'printer_print', label: '打印任务', kind: 'device', icon: 'Printer' },
    ]
  },
  {
    name: '逻辑节点',
    nodes: [
      { type: 'condition_router', label: '条件路由', kind: 'logic', icon: 'Share' },
      { type: 'order_type_router', label: '订单类型路由', kind: 'logic', icon: 'Switch' },
      { type: 'foreach', label: '循环遍历', kind: 'logic', icon: 'Refresh' },
      { type: 'fork_join', label: '并发分支', kind: 'logic', icon: 'Connection' },
      { type: 'wait_for_event', label: '等待事件', kind: 'logic', icon: 'Timer' },
    ]
  },
  {
    name: '数据节点',
    nodes: [
      { type: 'chute_query', label: '查询格口', kind: 'data', icon: 'Search' },
      { type: 'order_query', label: '查询订单', kind: 'data', icon: 'Tickets' },
      { type: 'order_create', label: '创建订单', kind: 'data', icon: 'CirclePlus' },
      { type: 'order_update_status', label: '更新订单状态', kind: 'data', icon: 'Edit' },
      { type: 'wave_query', label: '查询波次', kind: 'data', icon: 'List' },
    ]
  },
  {
    name: '系统节点',
    nodes: [
      { type: 'send_email', label: '发送邮件', kind: 'system', icon: 'Message' },
      { type: 'log_record', label: '记录日志', kind: 'system', icon: 'Document' },
      { type: 'file_upload', label: '文件上传', kind: 'system', icon: 'Upload' },
    ]
  },
])

const filteredCategories = computed(() => {
  if (!searchText.value) return categories.value
  return categories.value.map(cat => ({
    ...cat,
    nodes: cat.nodes.filter(node => 
      node.label.toLowerCase().includes(searchText.value.toLowerCase())
    )
  })).filter(cat => cat.nodes.length > 0)
})

const onDragStart = (event: DragEvent, node: any) => {
  if (event.dataTransfer) {
    event.dataTransfer.setData('application/vueflow', JSON.stringify(node))
    event.dataTransfer.effectAllowed = 'move'
  }
}
</script>

<style scoped>
.node-library {
  width: 260px;
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
}

.library-content {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.node-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.node-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  border-radius: 6px;
  cursor: move;
  transition: all 0.2s;
  border-left: 3px solid transparent;
}

.node-item:hover {
  background: #f5f7fa;
}

.node-device {
  border-left-color: #409EFF;
}

.node-logic {
  border-left-color: #67C23A;
}

.node-data {
  border-left-color: #E6A23C;
}

.node-system {
  border-left-color: #909399;
}

.node-label {
  font-size: 13px;
  color: #606266;
}
</style>
