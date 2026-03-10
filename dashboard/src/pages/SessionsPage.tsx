import { useQuery } from '@tanstack/react-query'
import { fetchSessions } from '../api/sessions'
import { Badge } from '../components/ui/Badge'
import type { Session } from '../types/api'

export default function SessionsPage() {
  const { data: sessions = [], isLoading } = useQuery<Session[]>({
    queryKey: ['sessions'],
    queryFn: fetchSessions,
  })

  if (isLoading) return <div>Loading...</div>

  return (
    <div>
      <h1 style={{ fontFamily: 'var(--font-mono)', marginBottom: 'var(--gap-lg)' }}>Sessions</h1>
      <table style={{ width: '100%', borderCollapse: 'collapse', fontFamily: 'var(--font-mono)', fontSize: 'var(--font-size-sm)' }}>
        <thead>
          <tr style={{ borderBottom: '1px solid var(--border)', color: 'var(--text-muted)', textAlign: 'left' }}>
            <th style={{ padding: '8px 12px' }}>Started</th>
            <th style={{ padding: '8px 12px' }}>Channel</th>
            <th style={{ padding: '8px 12px' }}>Provider</th>
            <th style={{ padding: '8px 12px' }}>Messages</th>
          </tr>
        </thead>
        <tbody>
          {sessions.map(s => (
            <tr key={s.id} style={{ borderBottom: '1px solid var(--border)' }}>
              <td style={{ padding: '8px 12px' }}>{new Date(s.startedAt).toLocaleString()}</td>
              <td style={{ padding: '8px 12px' }}><Badge label={s.channelType} variant="info" /></td>
              <td style={{ padding: '8px 12px' }}>{s.provider}</td>
              <td style={{ padding: '8px 12px' }}>{s.messageCount}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}
