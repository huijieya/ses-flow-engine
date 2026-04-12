import axios from 'axios'

const api = axios.create({
  baseURL: '/api/v1',
  timeout: 30000,
  headers: {
    'Content-Type': 'application/json',
  },
})

// 请求拦截器
api.interceptors.request.use(
  (config) => {
    console.log(`[API Request] ${config.method?.toUpperCase()} ${config.url}`)
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

// 响应拦截器
api.interceptors.response.use(
  (response) => {
    console.log(`[API Response] ${response.config.url}`, response.data)
    // 统一包装返回格式
    return {
      data: response.data,
      status: response.status,
      message: 'success',
    }
  },
  (error) => {
    console.error('[API Error]:', error)
    if (error.response) {
      // 服务器返回错误
      const message = error.response.data?.message || `请求失败: ${error.response.status}`
      console.error('Error message:', message)
    } else if (error.request) {
      // 请求发送失败（网络问题等）
      console.error('Network error: 无法连接到后端服务器')
    }
    return Promise.reject(error)
  }
)

export default api

export * from './flows'
export * from './nodes'
export * from './devices'
export * from './apps'
