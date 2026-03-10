import { apiFetch } from './client'
import { mockMessages } from './mock'
import type { ChatMessage } from '../types/api'

const USE_MOCK = import.meta.env.VITE_USE_MOCK === 'true'

export async function fetchHistory(): Promise<ChatMessage[]> {
  if (USE_MOCK) return [...mockMessages]
  return apiFetch<ChatMessage[]>('/api/v1/chat/history')
}

export async function sendMessage(content: string): Promise<ChatMessage> {
  if (USE_MOCK) {
    return { id: Date.now().toString(), role: 'assistant', content: `Echo: ${content}`, timestamp: new Date().toISOString() }
  }
  return apiFetch<ChatMessage>('/api/v1/chat', { method: 'POST', body: JSON.stringify({ content }) })
}
