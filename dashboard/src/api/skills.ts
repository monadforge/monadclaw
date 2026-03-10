import { apiFetch } from './client'
import { mockSkills } from './mock'
import type { Skill } from '../types/api'

const USE_MOCK = import.meta.env.VITE_USE_MOCK === 'true'

export async function fetchSkills(): Promise<Skill[]> {
  if (USE_MOCK) return mockSkills
  return apiFetch<Skill[]>('/api/v1/skills')
}
