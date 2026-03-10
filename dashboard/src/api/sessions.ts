import { apiFetch } from './client'
import { mockSessions } from './mock'
import type { Session } from '../types/api'

const USE_MOCK = import.meta.env.VITE_USE_MOCK === 'true'

export async function fetchSessions(): Promise<Session[]> {
  if (USE_MOCK) return mockSessions
  return apiFetch<Session[]>('/api/v1/chat/history')
}
