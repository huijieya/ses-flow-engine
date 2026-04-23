import api from './index'
import type { App } from '@/types'

export const getApps = () => api.get('/apps')

export const getApp = (id: string) => api.get(`/apps/${id}`)

export const createApp = (data: Partial<App>) => api.post('/apps', data)

export const updateApp = (id: string, data: Partial<App>) => api.put(`/apps/${id}`, data)

export const deleteApp = (id: string) => api.delete(`/apps/${id}`)
