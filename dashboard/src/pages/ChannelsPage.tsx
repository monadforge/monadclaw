import { useQuery } from '@tanstack/react-query'
import { fetchChannels } from '../api/channels'
import { Card } from '../components/ui/Card'
import { Badge } from '../components/ui/Badge'
import { Button } from '../components/ui/Button'
import type { Channel } from '../types/api'

export default function ChannelsPage() {
  const { data: channels = [], isLoading } = useQuery<Channel[]>({
    queryKey: ['channels'],
    queryFn: fetchChannels,
  })

  if (isLoading) return <div>Loading...</div>

  return (
    <div>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 'var(--gap-lg)' }}>
        <h1 style={{ fontFamily: 'var(--font-mono)' }}>Channels</h1>
      </div>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--gap-sm)' }}>
        {channels.map(ch => (
          <Card key={ch.id}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 'var(--gap)' }}>
              <Badge label={ch.type} variant="info" />
              <span style={{ fontFamily: 'var(--font-mono)', flex: 1 }}>{ch.name}</span>
              <Badge label={ch.enabled ? 'enabled' : 'disabled'} variant={ch.enabled ? 'success' : 'neutral'} />
              <Button variant="ghost">Configure</Button>
            </div>
          </Card>
        ))}
      </div>
    </div>
  )
}
