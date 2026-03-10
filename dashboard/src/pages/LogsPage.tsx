import { useLogs } from '../hooks/useLogs'
import { Button } from '../components/ui/Button'
import './LogsPage.css'

const LEVELS = ['ALL', 'DEBUG', 'INFO', 'WARN', 'ERROR'] as const

export default function LogsPage() {
  const { logs, level, setLevel, search, setSearch } = useLogs()

  return (
    <div>
      <h1 style={{ fontFamily: 'var(--font-mono)', marginBottom: 'var(--gap-lg)' }}>Logs</h1>
      <div className="logs-toolbar">
        {LEVELS.map(l => (
          <Button
            key={l}
            variant={level === l ? 'primary' : 'ghost'}
            onClick={() => setLevel(l)}
          >
            {l}
          </Button>
        ))}
        <input
          className="logs-search"
          placeholder="Search logs..."
          value={search}
          onChange={e => setSearch(e.target.value)}
        />
      </div>
      <div className="log-list">
        {logs.map((entry, i) => (
          <div key={i} className="log-entry" data-level={entry.level}>
            <span className="log-time">{new Date(entry.timestamp).toLocaleTimeString()}</span>
            <span className="log-level">{entry.level}</span>
            <span className="log-message">{entry.message}</span>
          </div>
        ))}
        {logs.length === 0 && (
          <div style={{ color: 'var(--text-muted)', padding: 'var(--gap)' }}>No logs match filter.</div>
        )}
      </div>
    </div>
  )
}
