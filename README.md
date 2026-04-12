# SES - Smart Execution System

智能执行系统 - 面向仓储物流场景的流程编排与节点执行系统。

## 系统架构

```
┌─────────────────────────────────────────────────────────────────┐
│                        应用层 (Frontend Vue3)                    │
├─────────────────────────────────────────────────────────────────┤
│                    工作流编排层 (Flow Engine)                      │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  DAG引擎 | 工作流实例管理 | 并发分支(Fork/Join) | 条件分支 │   │
│  └─────────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│                      节点执行层 (Executor)                       │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ 节点执行器 | 输入验证 | 输出注入 | 重试机制 | 超时控制  │   │
│  └─────────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│                      节点实现层 (Implementation)                 │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ │
│  │设备节点 │ │逻辑节点 │ │数据节点 │ │查询节点 │ │系统节点 │ │
│  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘ │
├─────────────────────────────────────────────────────────────────┤
│                    事件总线层 (Event Bus)                        │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  Redis Streams | 事件发布订阅 | 持久化队列 | 断线重连   │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

## 技术栈

### 后端 (Rust)
- **Web框架**: Axum
- **数据库**: PostgreSQL + SQLx
- **缓存**: Redis
- **异步运行时**: Tokio

### 前端 (Vue3)
- **UI框架**: Element Plus
- **状态管理**: Pinia
- **流程编排**: Vue Flow (@vue-flow/core)
- **构建工具**: Vite

## 项目结构

```
ses-flow-orchestrator/
├── backend/                 # Rust后端
│   ├── src/
│   │   ├── api/            # API路由
│   │   ├── core/           # 核心模块 (config, error, state, types)
│   │   ├── devices/        # 设备接入层
│   │   ├── engine/         # 工作流引擎 (DAG, Executor, Scheduler)
│   │   ├── events/         # 事件总线
│   │   ├── models/         # 数据模型
│   │   ├── nodes/          # 节点实现
│   │   ├── utils/          # 工具函数
│   │   └── main.rs         # 入口文件
│   ├── migrations/         # 数据库迁移
│   └── Cargo.toml
│
├── frontend/               # Vue3前端
│   ├── src/
│   │   ├── api/            # API客户端
│   │   ├── components/     # 组件
│   │   │   ├── FlowCanvas.vue
│   │   │   ├── NodeLibrary.vue
│   │   │   └── PropertyPanel.vue
│   │   ├── router/         # 路由配置
│   │   ├── stores/         # Pinia状态
│   │   ├── types/          # TypeScript类型
│   │   ├── views/          # 页面视图
│   │   │   ├── FlowEditor.vue
│   │   │   ├── Flows.vue
│   │   │   ├── Nodes.vue
│   │   │   └── Devices.vue
│   │   └── main.ts
│   ├── package.json
│   └── vite.config.ts
│
└── README.md
```

## 快速开始

### 1. 环境准备

- Rust 1.75+
- Node.js 18+
- PostgreSQL 14+
- Redis 7+

### 2. 数据库初始化

```bash
# 创建数据库
createdb ses_db

# 运行迁移
# (通过后端启动时自动执行)
```

### 3. 启动后端

```bash
cd backend

# 设置环境变量 (或在 .env 文件中)
export SES__DATABASE__URL="postgres://user:password@localhost/ses_db"
export SES__REDIS__URL="redis://localhost:6379"

# 运行
cargo run
```

后端服务将启动在 `http://localhost:8080`

### 4. 启动前端

```bash
cd frontend

# 安装依赖
npm install

# 开发模式
npm run dev
```

前端将启动在 `http://localhost:3000`

## 核心功能

### 工作流编排

- **DAG支持**: 有向无环图工作流编排
- **节点类型**: 设备节点、逻辑节点、数据节点、查询节点、系统节点
- **条件分支**: 支持基于条件的工作流分支
- **并发执行**: Fork/Join 并发分支支持
- **事件驱动**: 基于事件的工作流驱动

### 节点库

| 节点类型 | 节点ID | 说明 |
|---------|--------|------|
| 设备节点 | chute_operate | 格口批量操作 |
| 设备节点 | device_command | 通用设备命令 |
| 设备节点 | printer_print | 打印任务 |
| 逻辑节点 | condition_router | 条件路由 |
| 逻辑节点 | order_type_router | 订单类型路由 |
| 逻辑节点 | fork_join | 并发分支 |
| 逻辑节点 | wait_for_event | 等待事件 |
| 数据节点 | chute_query | 查询格口 |
| 数据节点 | order_query | 查询订单 |
| 数据节点 | order_create | 创建订单 |
| 系统节点 | send_email | 发送邮件 |
| 系统节点 | log_record | 记录日志 |

### 设备接入

- **标准接口**: GET /device/{id}/status, POST /device/{id}/task
- **回调机制**: POST /ses/callback
- **多协议支持**: HTTP, WebSocket, MQTT

## API 文档

### 工作流 API

| 方法 | 路径 | 说明 |
|-----|------|------|
| GET | /api/v1/flows | 获取工作流列表 |
| POST | /api/v1/flows | 创建工作流 |
| GET | /api/v1/flows/:id | 获取工作流详情 |
| PUT | /api/v1/flows/:id | 更新工作流 |
| DELETE | /api/v1/flows/:id | 删除工作流 |
| POST | /api/v1/flows/:id/execute | 执行工作流 |
| POST | /api/v1/flows/:instance_id/resume | 恢复工作流 |
| POST | /api/v1/flows/:instance_id/cancel | 取消工作流 |

### 设备 API

| 方法 | 路径 | 说明 |
|-----|------|------|
| GET | /api/v1/devices | 获取设备列表 |
| POST | /api/v1/devices | 创建设备 |
| GET | /api/v1/devices/:id | 获取设备详情 |
| POST | /api/v1/devices/:id/task | 下发任务 |
| POST | /ses/callback | 设备回调 |

## 数据模型

### 工作流定义 (Flow)

```json
{
  "id": "uuid",
  "name": "工作流名称",
  "flow_json": {
    "nodes": [...],
    "edges": [...]
  },
  "status": "DRAFT|PUBLISHED|ARCHIVED",
  "version": 1
}
```

### 节点定义 (Node)

```json
{
  "id": "node_xxx",
  "node_def_id": "chute_operate",
  "name": "格口操作",
  "kind": "device",
  "position": {"x": 100, "y": 200},
  "config": {...}
}
```

## 开发计划

- [x] 项目基础架构搭建
- [x] DAG 工作流引擎核心
- [x] 节点执行器框架
- [x] 基础节点实现
- [x] 事件总线
- [x] 前端工作流编排器
- [ ] 节点扩展机制
- [ ] 设备协议适配器
- [ ] 工作流模板库
- [ ] 监控和告警

## 贡献指南

欢迎提交 Issue 和 PR！

## License

MIT
