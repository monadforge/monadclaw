import type {
  AgentStatus, ChatMessage, Channel, Session,
  LogEntry, UsageStat, CronJob, Skill
} from '../types/api'

export const mockStatus: AgentStatus = {
  status: 'online',
  provider: 'openai',
  model: 'gpt-4o',
  memoryShortTerm: 12,
  memoryLongTerm: 347,
  uptimeSeconds: 86400,
  messagesToday: 42,
}

export const mockMessages: ChatMessage[] = [
  { id: '1', role: 'user', content: 'Hello agent', timestamp: new Date().toISOString() },
  { id: '2', role: 'assistant', content: 'Hello! How can I help?', timestamp: new Date().toISOString() },
]

export const mockChannels: Channel[] = [
  { id: 'discord-1', type: 'discord', name: 'Main Guild', enabled: true, config: {} },
  { id: 'rest-1',    type: 'rest',    name: 'REST API',   enabled: true, config: {} },
]

export const mockSessions: Session[] = [
  { id: 's1', channelType: 'discord', messageCount: 12, provider: 'openai', startedAt: new Date(Date.now() - 3600000).toISOString() },
  { id: 's2', channelType: 'rest',    messageCount: 5,  provider: 'openai', startedAt: new Date(Date.now() - 7200000).toISOString() },
]

export const mockLogs: LogEntry[] = [
  { timestamp: new Date().toISOString(), level: 'INFO',  message: 'Agent started', target: 'core' },
  { timestamp: new Date().toISOString(), level: 'DEBUG', message: 'Memory loaded: 347 entries', target: 'memory' },
  { timestamp: new Date().toISOString(), level: 'WARN',  message: 'Provider rate limit approaching', target: 'providers' },
]

export const mockUsage: UsageStat[] = [
  { date: '2026-03-08', provider: 'openai', model: 'gpt-4o', inputTokens: 12000, outputTokens: 4000, estimatedCostUsd: 0.18 },
  { date: '2026-03-09', provider: 'openai', model: 'gpt-4o', inputTokens: 15000, outputTokens: 5000, estimatedCostUsd: 0.225 },
  { date: '2026-03-10', provider: 'openai', model: 'gpt-4o', inputTokens: 8000,  outputTokens: 3000, estimatedCostUsd: 0.12 },
]

export const mockCrons: CronJob[] = [
  { id: 'c1', schedule: '0 * * * *', description: 'Hourly health check', enabled: true, lastStatus: 'ok', nextRun: new Date(Date.now() + 1800000).toISOString() },
  { id: 'c2', schedule: '0 0 * * *', description: 'Daily summary', enabled: false, nextRun: new Date(Date.now() + 86400000).toISOString() },
]

export const mockSkills: Skill[] = [
  { id: 'web-search', name: 'web-search', version: '1.0.0', enabled: true, description: 'Search the web for information' },
  { id: 'code-exec',  name: 'code-exec',  version: '0.9.0', enabled: false, description: 'Execute code snippets in sandbox' },
]

export const mockConfig: Record<string, unknown> = {
  provider: 'openai',
  model: 'gpt-4o',
  memory: { shortTermLimit: 20, longTermStore: 'sqlite' },
  api: { port: 3000, rateLimit: 60 },
}
