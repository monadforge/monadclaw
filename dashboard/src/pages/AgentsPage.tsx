import { useState } from 'react'
import { Button } from '../components/ui/Button'
import { Card } from '../components/ui/Card'

type Tab = 'tools' | 'files' | 'config' | 'channels'

export default function AgentsPage() {
  const [activeTab, setActiveTab] = useState<Tab>('tools')

  return (
    <div>
      <h1 style={{ fontFamily: 'var(--font-mono)', marginBottom: 'var(--gap-lg)' }}>Agent</h1>
      <div style={{ display: 'flex', gap: 'var(--gap-sm)', marginBottom: 'var(--gap)' }}>
        {(['tools', 'files', 'config', 'channels'] as Tab[]).map(t => (
          <Button
            key={t}
            variant={activeTab === t ? 'primary' : 'ghost'}
            onClick={() => setActiveTab(t)}
          >
            {t}
          </Button>
        ))}
      </div>
      <Card title={activeTab.toUpperCase()}>
        <div style={{ color: 'var(--text-muted)', fontFamily: 'var(--font-mono)' }}>
          {activeTab} tab — connect to /api/v1/agents endpoint
        </div>
      </Card>
    </div>
  )
}
