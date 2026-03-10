import { apiFetch } from './client'
import { mockLogs } from './mock'
import type { LogEntry } from '../types/api'

const USE_MOCK = import.meta.env.VITE_USE_MOCK === 'true'

export async function fetchLogs(): Promise<LogEntry[]> {
  if (USE_MOCK) return mockLogs
  return apiFetch<LogEntry[]>('/api/v1/logs')
}
