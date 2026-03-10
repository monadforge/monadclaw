import { useState } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import Editor from '@monaco-editor/react'
import { fetchConfig, updateConfig } from '../api/config'
import { Button } from '../components/ui/Button'

export default function ConfigPage() {
  const qc = useQueryClient()
  const { data, isLoading } = useQuery({
    queryKey: ['config'],
    queryFn: fetchConfig,
  })
  const [draft, setDraft] = useState<string | undefined>()
  const [error, setError] = useState('')

  const mutation = useMutation({
    mutationFn: (value: string) => {
      const parsed = JSON.parse(value)
      return updateConfig(parsed)
    },
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ['config'] })
      setDraft(undefined)
      setError('')
    },
    onError: (e: unknown) => {
      setError(e instanceof Error ? e.message : 'Save failed')
    },
  })

  if (isLoading) return <div>Loading...</div>

  const value = draft ?? JSON.stringify(data, null, 2)

  return (
    <div>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 'var(--gap-lg)' }}>
        <h1 style={{ fontFamily: 'var(--font-mono)' }}>Config</h1>
        <div style={{ display: 'flex', gap: 'var(--gap-sm)' }}>
          <Button variant="ghost" onClick={() => { setDraft(undefined); setError('') }}>Discard</Button>
          <Button
            variant="primary"
            disabled={mutation.isPending}
            onClick={() => mutation.mutate(value)}
          >
            {mutation.isPending ? 'Saving…' : 'Save'}
          </Button>
        </div>
      </div>
      {error && (
        <div style={{ color: 'var(--error)', fontFamily: 'var(--font-mono)', marginBottom: 'var(--gap)', fontSize: 'var(--font-size-sm)' }}>
          {error}
        </div>
      )}
      <div style={{ border: '1px solid var(--border)', borderRadius: 'var(--radius)', overflow: 'hidden' }}>
        <Editor
          height="70vh"
          language="json"
          theme="vs-dark"
          value={value}
          onChange={v => setDraft(v)}
          options={{
            fontSize: 13,
            fontFamily: 'JetBrains Mono, monospace',
            minimap: { enabled: false },
            scrollBeyondLastLine: false,
            tabSize: 2,
          }}
        />
      </div>
    </div>
  )
}
