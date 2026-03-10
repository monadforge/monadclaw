import { useStatus } from '../hooks/useStatus'
import { Card } from '../components/ui/Card'
import { Badge } from '../components/ui/Badge'
import './OverviewPage.css'

function formatUptime(seconds: number) {
  const h = Math.floor(seconds / 3600)
  const m = Math.floor((seconds % 3600) / 60)
  return `${h}h ${m}m`
}

export default function OverviewPage() {
  const { data, isLoading, error } = useStatus()

  if (isLoading) return <div>Loading...</div>
  if (error || !data) return <div>Failed to load status</div>

  const statusVariant =
    data.status === 'online' ? 'success' :
    data.status === 'degraded' ? 'warning' : 'error'

  return (
    <div>
      <h1 style={{ fontFamily: 'var(--font-mono)', marginBottom: 'var(--gap-lg)' }}>
        Overview
      </h1>
      <div className="overview-grid">
        <Card title="Agent Status">
          <Badge label={data.status} variant={statusVariant} />
        </Card>
        <Card title="Provider">
          <div className="stat-value">{data.model}</div>
          <div className="stat-label">{data.provider}</div>
        </Card>
        <Card title="Memory">
          <div className="stat-value">{data.memoryShortTerm}</div>
          <div className="stat-label">Short-term items</div>
          <div className="stat-value" style={{ marginTop: 8 }}>{data.memoryLongTerm}</div>
          <div className="stat-label">Long-term entries</div>
        </Card>
        <Card title="Messages Today">
          <div className="stat-value">{data.messagesToday}</div>
        </Card>
        <Card title="Uptime">
          <div className="stat-value">{formatUptime(data.uptimeSeconds)}</div>
        </Card>
      </div>
    </div>
  )
}
