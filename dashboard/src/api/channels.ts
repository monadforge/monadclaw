import { apiFetch } from './client'
import { mockChannels } from './mock'
import type { Channel } from '../types/api'

const USE_MOCK = import.meta.env.VITE_USE_MOCK === 'true'

export async function fetchChannels(): Promise<Channel[]> {
  if (USE_MOCK) return mockChannels
  return apiFetch<Channel[]>('/api/v1/channels')
}
