<template>
  <div class="nodes-page">
    <div class="page-header">
      <h2>节点库</h2>
      <el-button type="primary">
        <el-icon><Plus /></el-icon>新建节点
      </el-button>
    </div>

    <el-card>
      <el-tabs v-model="activeTab">
        <el-tab-pane v-for="kind in nodeKinds" :key="kind" :label="kind" :name="kind">
          <el-table :data="getNodesByKind(kind)" style="width: 100%">
            <el-table-column prop="name" label="名称" min-width="150" />
            <el-table-column prop="description" label="描述" min-width="250" show-overflow-tooltip />
            <el-table-column prop="category" label="分类" width="120" />
            <el-table-column label="操作" width="150">
              <template #default="{ row }">
                <el-button type="primary" link>查看</el-button>
                <el-button type="danger" link>删除</el-button>
              </template>
            </el-table-column>
          </el-table>
        </el-tab-pane>
      </el-tabs>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const activeTab = ref('设备节点')
const nodeKinds = ['设备节点', '逻辑节点', '数据节点', '查询节点', '系统节点']

const nodes = ref([
  { name: '格口操作', description: '批量操作格口（打开/关闭/禁用/启用）', kind: '设备节点', category: '设备控制' },
  { name: '设备命令', description: '向设备发送通用命令', kind: '设备节点', category: '设备控制' },
  { name: '条件路由', description: '根据条件表达式决定流程走向', kind: '逻辑节点', category: '流程控制' },
  { name: '循环遍历', description: '对集合中的每个元素执行子流程', kind: '逻辑节点', category: '流程控制' },
  { name: '查询格口', description: '查询格口信息和状态', kind: '数据节点', category: '数据查询' },
  { name: '创建订单', description: '创建新订单记录', kind: '数据节点', category: '数据操作' },
  { name: '发送邮件', description: '发送邮件通知', kind: '系统节点', category: '系统操作' },
])

const getNodesByKind = (kind: string) => {
  return nodes.value.filter(n => n.kind === kind)
}
</script>

<style scoped>
.nodes-page {
  padding: 20px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}
</style>
