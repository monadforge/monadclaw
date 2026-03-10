import { apiFetch } from './client'

const USE_MOCK = import.meta.env.VITE_USE_MOCK === 'true'

export async function fetchAgent(): Promise<Record<string, unknown>> {
  if (USE_MOCK) return { id: 'default', name: 'monadclaw', status: 'online' }
  return apiFetch<Record<string, unknown>>('/api/v1/agents/default')
}
