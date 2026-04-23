export interface Flow {
  id: string
  name: string
  description: string
  version: number
  status: 'DRAFT' | 'PUBLISHED' | 'ARCHIVED'
  flowJson: FlowDefinition
  createdAt: string
  updatedAt: string
}

export interface FlowDefinition {
  nodes: FlowNode[]
  edges: FlowEdge[]
  variables?: Record<string, any>
}

export interface FlowNode {
  id: string
  nodeDefId: string
  name: string
  kind: 'device' | 'logic' | 'data' | 'query' | 'system'
  position: { x: number; y: number }
  config: Record<string, any>
}

export interface FlowEdge {
  id: string
  source: string
  target: string
  condition?: string
}

export interface NodeDefinition {
  id: string
  nodeId: string
  name: string
  description: string
  kind: 'device' | 'logic' | 'data' | 'query' | 'system'
  deviceType?: string
  inputSchema?: any
  outputSchema?: any
  configSchema?: any
  icon?: string
  color?: string
  category: string
}

export interface Device {
  id: string
  deviceId: string
  deviceName: string
  deviceType: string
  online: boolean
  status: string
  lastHeartbeat?: string
}

export interface App {
  id: string
  name: string
  description: string
  status: 'ACTIVE' | 'INACTIVE' | 'SUSPENDED'
  createdAt: string
}

export interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: {
    code: number
    message: string
  }
}
