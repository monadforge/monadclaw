import { apiFetch } from './client'
import { mockStatus } from './mock'
import type { AgentStatus } from '../types/api'

const USE_MOCK = import.meta.env.VITE_USE_MOCK === 'true'

export async function fetchStatus(): Promise<AgentStatus> {
  if (USE_MOCK) return mockStatus
  return apiFetch<AgentStatus>('/api/v1/status')
}
