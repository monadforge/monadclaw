import { useQuery } from '@tanstack/react-query'
import { fetchUsage } from '../api/usage'
import { BarChart, Bar, XAxis, YAxis, Tooltip, ResponsiveContainer } from 'recharts'
import { Card } from '../components/ui/Card'
import type { UsageStat } from '../types/api'

export default function UsagePage() {
  const { data: stats = [], isLoading } = useQuery<UsageStat[]>({
    queryKey: ['usage'],
    queryFn: fetchUsage,
  })

  if (isLoading) return <div>Loading...</div>

  const totalCost = stats.reduce((sum, s) => sum + s.estimatedCostUsd, 0)
  const totalTokens = stats.reduce((sum, s) => sum + s.inputTokens + s.outputTokens, 0)

  return (
    <div>
      <h1 style={{ fontFamily: 'var(--font-mono)', marginBottom: 'var(--gap-lg)' }}>Usage</h1>
      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 'var(--gap)', marginBottom: 'var(--gap-lg)' }}>
        <Card title="Total Tokens">
          <div style={{ fontFamily: 'var(--font-mono)', fontSize: 'var(--font-size-xl)', color: 'var(--accent)' }}>
            {totalTokens.toLocaleString()}
          </div>
        </Card>
        <Card title="Estimated Cost">
          <div style={{ fontFamily: 'var(--font-mono)', fontSize: 'var(--font-size-xl)', color: 'var(--accent)' }}>
            ${totalCost.toFixed(4)}
          </div>
        </Card>
      </div>
      <Card title="Tokens by Day">
        <ResponsiveContainer width="100%" height={250}>
          <BarChart data={stats}>
            <XAxis dataKey="date" stroke="var(--text-muted)" tick={{ fontSize: 11 }} />
            <YAxis stroke="var(--text-muted)" tick={{ fontSize: 11 }} />
            <Tooltip
              contentStyle={{ background: 'var(--surface-2)', border: '1px solid var(--border)', borderRadius: 8 }}
              labelStyle={{ color: 'var(--text)' }}
            />
            <Bar dataKey="inputTokens" stackId="a" fill="var(--info)" name="Input" />
            <Bar dataKey="outputTokens" stackId="a" fill="var(--accent)" name="Output" />
          </BarChart>
        </ResponsiveContainer>
      </Card>
    </div>
  )
}
