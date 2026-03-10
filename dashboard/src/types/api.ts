export interface ApiError {
  error: { code: string; message: string }
}

export interface AgentStatus {
  status: 'online' | 'offline' | 'degraded'
  provider: string
  model: string
  memoryShortTerm: number
  memoryLongTerm: number
  uptimeSeconds: number
  messagesToday: number
}

export interface ChatMessage {
  id: string
  role: 'user' | 'assistant' | 'tool'
  content: string
  toolName?: string
  timestamp: string
}

export interface Channel {
  id: string
  type: 'discord' | 'rest' | 'telegram'
  name: string
  enabled: boolean
  config: Record<string, unknown>
}

export interface Session {
  id: string
  channelType: string
  messageCount: number
  provider: string
  startedAt: string
  endedAt?: string
}

export interface LogEntry {
  timestamp: string
  level: 'DEBUG' | 'INFO' | 'WARN' | 'ERROR'
  message: string
  target?: string
}

export interface UsageStat {
  date: string
  provider: string
  model: string
  inputTokens: number
  outputTokens: number
  estimatedCostUsd: number
}

export interface CronJob {
  id: string
  schedule: string
  description: string
  enabled: boolean
  lastRun?: string
  lastStatus?: 'ok' | 'error'
  nextRun: string
}

export interface Skill {
  id: string
  name: string
  version: string
  enabled: boolean
  description: string
  configSchema?: Record<string, unknown>
}
