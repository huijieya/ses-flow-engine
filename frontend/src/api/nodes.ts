import api from './index'
import type { NodeDefinition } from '@/types'

export const getNodeDefinitions = () => api.get('/nodes/definitions')

export const getNodeDefinition = (id: string) => api.get(`/nodes/definitions/${id}`)

export const createNodeDefinition = (data: Partial<NodeDefinition>) => 
  api.post('/nodes/definitions', data)

export const updateNodeDefinition = (id: string, data: Partial<NodeDefinition>) => 
  api.put(`/nodes/definitions/${id}`, data)

export const deleteNodeDefinition = (id: string) => 
  api.delete(`/nodes/definitions/${id}`)
