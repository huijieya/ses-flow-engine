import api from './index'
import type { Flow, FlowDefinition } from '@/types'

export const getFlows = () => api.get('/flows')

export const getFlow = (id: string) => api.get(`/flows/${id}`)

export const createFlow = (data: Partial<Flow>) => api.post('/flows', data)

export const updateFlow = (id: string, data: Partial<Flow>) => api.put(`/flows/${id}`, data)

export const deleteFlow = (id: string) => api.delete(`/flows/${id}`)

export const executeFlow = (id: string, context?: any) => 
  api.post(`/flows/${id}/execute`, { initialContext: context })

export const resumeFlow = (instanceId: string) => 
  api.post(`/flows/${instanceId}/resume`)

export const cancelFlow = (instanceId: string) => 
  api.post(`/flows/${instanceId}/cancel`)
