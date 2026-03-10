import { apiFetch } from './client'
import { mockUsage } from './mock'
import type { UsageStat } from '../types/api'

const USE_MOCK = import.meta.env.VITE_USE_MOCK === 'true'

export async function fetchUsage(): Promise<UsageStat[]> {
  if (USE_MOCK) return mockUsage
  return apiFetch<UsageStat[]>('/api/v1/usage')
}
