import { apiFetch } from './client'
import { mockConfig } from './mock'

const USE_MOCK = import.meta.env.VITE_USE_MOCK === 'true'

export async function fetchConfig(): Promise<Record<string, unknown>> {
  if (USE_MOCK) return { ...mockConfig }
  return apiFetch<Record<string, unknown>>('/api/v1/config')
}

export async function updateConfig(data: Record<string, unknown>): Promise<void> {
  if (USE_MOCK) return
  return apiFetch<void>('/api/v1/config', { method: 'PATCH', body: JSON.stringify(data) })
}
