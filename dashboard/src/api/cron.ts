import { apiFetch } from './client'
import { mockCrons } from './mock'
import type { CronJob } from '../types/api'

const USE_MOCK = import.meta.env.VITE_USE_MOCK === 'true'

export async function fetchCron(): Promise<CronJob[]> {
  if (USE_MOCK) return mockCrons
  return apiFetch<CronJob[]>('/api/v1/cron')
}
