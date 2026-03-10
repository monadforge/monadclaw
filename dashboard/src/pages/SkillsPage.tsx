import { useQuery } from '@tanstack/react-query'
import { fetchSkills } from '../api/skills'
import { Card } from '../components/ui/Card'
import { Badge } from '../components/ui/Badge'
import { Button } from '../components/ui/Button'
import type { Skill } from '../types/api'

export default function SkillsPage() {
  const { data: skills = [], isLoading } = useQuery<Skill[]>({
    queryKey: ['skills'],
    queryFn: fetchSkills,
  })

  if (isLoading) return <div>Loading...</div>

  return (
    <div>
      <h1 style={{ fontFamily: 'var(--font-mono)', marginBottom: 'var(--gap-lg)' }}>Skills</h1>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--gap-sm)' }}>
        {skills.map(skill => (
          <Card key={skill.id}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 'var(--gap)' }}>
              <div style={{ flex: 1 }}>
                <div style={{ fontFamily: 'var(--font-mono)', marginBottom: 4 }}>
                  {skill.name} <span style={{ color: 'var(--text-muted)', fontSize: 'var(--font-size-xs)' }}>v{skill.version}</span>
                </div>
                <div style={{ fontSize: 'var(--font-size-sm)', color: 'var(--text-muted)' }}>{skill.description}</div>
              </div>
              <Badge label={skill.enabled ? 'enabled' : 'disabled'} variant={skill.enabled ? 'success' : 'neutral'} />
              <Button variant="ghost">Configure</Button>
            </div>
          </Card>
        ))}
      </div>
    </div>
  )
}
