import { useQuery } from '@tanstack/react-query'
import { fetchCron } from '../api/cron'
import { Badge } from '../components/ui/Badge'
import { Button } from '../components/ui/Button'
import type { CronJob } from '../types/api'

export default function CronPage() {
  const { data: jobs = [], isLoading } = useQuery<CronJob[]>({
    queryKey: ['cron'],
    queryFn: fetchCron,
  })

  if (isLoading) return <div>Loading...</div>

  return (
    <div>
      <h1 style={{ fontFamily: 'var(--font-mono)', marginBottom: 'var(--gap-lg)' }}>Cron Jobs</h1>
      <table style={{ width: '100%', borderCollapse: 'collapse', fontFamily: 'var(--font-mono)', fontSize: 'var(--font-size-sm)' }}>
        <thead>
          <tr style={{ borderBottom: '1px solid var(--border)', color: 'var(--text-muted)', textAlign: 'left' }}>
            <th style={{ padding: '8px 12px' }}>Schedule</th>
            <th style={{ padding: '8px 12px' }}>Description</th>
            <th style={{ padding: '8px 12px' }}>Next Run</th>
            <th style={{ padding: '8px 12px' }}>Last Status</th>
            <th style={{ padding: '8px 12px' }}>Enabled</th>
          </tr>
        </thead>
        <tbody>
          {jobs.map(job => (
            <tr key={job.id} style={{ borderBottom: '1px solid var(--border)' }}>
              <td style={{ padding: '8px 12px', color: 'var(--accent)' }}>{job.schedule}</td>
              <td style={{ padding: '8px 12px' }}>{job.description}</td>
              <td style={{ padding: '8px 12px', color: 'var(--text-muted)' }}>{new Date(job.nextRun).toLocaleString()}</td>
              <td style={{ padding: '8px 12px' }}>
                {job.lastStatus && (
                  <Badge label={job.lastStatus} variant={job.lastStatus === 'ok' ? 'success' : 'error'} />
                )}
              </td>
              <td style={{ padding: '8px 12px' }}>
                <Button variant={job.enabled ? 'primary' : 'ghost'} style={{ padding: '3px 10px' }}>
                  {job.enabled ? 'On' : 'Off'}
                </Button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}
