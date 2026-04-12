import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Flow, FlowDefinition } from '@/types'

export const useFlowStore = defineStore('flow', () => {
  const flows = ref<Flow[]>([])
  const currentFlow = ref<Flow | null>(null)
  const loading = ref(false)

  const setFlows = (newFlows: Flow[]) => {
    flows.value = newFlows
  }

  const setCurrentFlow = (flow: Flow | null) => {
    currentFlow.value = flow
  }

  const createEmptyFlow = (): Flow => {
    return {
      id: '',
      name: '未命名工作流',
      description: '',
      version: 1,
      status: 'DRAFT',
      flowJson: {
        nodes: [],
        edges: [],
      },
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    }
  }

  return {
    flows,
    currentFlow,
    loading,
    setFlows,
    setCurrentFlow,
    createEmptyFlow,
  }
})
