import api from './index'
import type { Device } from '@/types'

export const getDevices = () => api.get('/devices')

export const getDevice = (id: string) => api.get(`/devices/${id}`)

export const createDevice = (data: Partial<Device>) => api.post('/devices', data)

export const updateDevice = (id: string, data: Partial<Device>) => 
  api.put(`/devices/${id}`, data)

export const deleteDevice = (id: string) => api.delete(`/devices/${id}`)

export const getDeviceStatus = (id: string) => api.get(`/devices/${id}/status`)

export const sendTask = (id: string, task: any) => 
  api.post(`/devices/${id}/task`, task)
