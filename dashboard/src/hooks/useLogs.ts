import { useQuery } from '@tanstack/react-query'
import { fetchLogs } from '../api/logs'
import { useState } from 'react'
import type { LogEntry } from '../types/api'

type Level = 'ALL' | 'DEBUG' | 'INFO' | 'WARN' | 'ERROR'

export function useLogs() {
  const [level, setLevel] = useState<Level>('ALL')
  const [search, setSearch] = useState('')

  const { data = [] } = useQuery<LogEntry[]>({
    queryKey: ['logs'],
    queryFn: fetchLogs,
    refetchInterval: 3_000,
  })

  const filtered = data.filter(e =>
    (level === 'ALL' || e.level === level) &&
    (search === '' || e.message.toLowerCase().includes(search.toLowerCase()))
  )

  return { logs: filtered, level, setLevel, search, setSearch }
}
