// Chat API — history endpoint only.
// Message sending is handled by useChat via fetch streaming.
import { apiFetch } from './client'
import { mockMessages } from './mock'
import type { ChatMessage } from '../types/api'

const USE_MOCK = import.meta.env.VITE_USE_MOCK === 'true'

export async function fetchHistory(): Promise<ChatMessage[]> {
  if (USE_MOCK) return [...mockMessages]
  return apiFetch<ChatMessage[]>('/api/v1/chat/history')
}
