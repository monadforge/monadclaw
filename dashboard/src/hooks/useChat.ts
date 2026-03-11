import { useState, useCallback } from 'react'
import type { ChatMessage } from '../types/api'

let _idCounter = 0
const nextId = () => String(++_idCounter)

export function useChat() {
  const [messages, setMessages] = useState<ChatMessage[]>([])
  const [isPending, setIsPending] = useState(false)

  const send = useCallback(
    (text: string) => {
      const userMsg: ChatMessage = {
        id: nextId(),
        role: 'user',
        content: text,
        timestamp: new Date().toISOString(),
      }
      const assistantId = nextId()
      const assistantMsg: ChatMessage = {
        id: assistantId,
        role: 'assistant',
        content: '',
        timestamp: new Date().toISOString(),
      }

      setMessages(prev => [...prev, userMsg, assistantMsg])
      setIsPending(true)

      const payload = {
        messages: [...messages, userMsg].map(({ role, content }) => ({ role, content })),
      }

      fetch('/api/v1/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      })
        .then(response => {
          if (!response.ok || !response.body) {
            setIsPending(false)
            return
          }

          const reader = response.body.getReader()
          const decoder = new TextDecoder()
          let buffer = ''

          const processLines = (chunk: string) => {
            buffer += chunk
            const lines = buffer.split('\n')
            buffer = lines.pop() ?? ''

            for (const line of lines) {
              if (!line.startsWith('data:')) continue
              const data = line.slice(5).replace(/^ /, '')
              if (data === '[DONE]' || data.startsWith('[ERROR]')) {
                setIsPending(false)
                return
              }
              setMessages(prev =>
                prev.map(m =>
                  m.id === assistantId ? { ...m, content: m.content + data } : m,
                ),
              )
            }
          }

          const read = (): Promise<void> =>
            reader.read().then(({ done, value }) => {
              if (done) {
                setIsPending(false)
                return
              }
              processLines(decoder.decode(value, { stream: true }))
              return read()
            })

          return read()
        })
        .catch(() => setIsPending(false))
    },
    [messages],
  )

  return { messages, send, isPending }
}
