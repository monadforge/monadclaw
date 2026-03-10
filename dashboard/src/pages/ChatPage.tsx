import { useState, useRef, useEffect } from 'react'
import type { KeyboardEvent } from 'react'
import { useChat } from '../hooks/useChat'
import { Button } from '../components/ui/Button'
import './ChatPage.css'

export default function ChatPage() {
  const { messages, send, isPending } = useChat()
  const [input, setInput] = useState('')
  const bottomRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [messages])

  const handleSend = async () => {
    const text = input.trim()
    if (!text || isPending) return
    setInput('')
    await send(text)
  }

  const handleKeyDown = (e: KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      void handleSend()
    }
  }

  return (
    <div className="chat-shell">
      <div className="chat-messages">
        {messages.map(msg => (
          <div key={msg.id} className={`chat-bubble ${msg.role}`}>
            {msg.content}
          </div>
        ))}
        {isPending && (
          <div className="chat-bubble assistant">
            <span className="cursor">█</span>
          </div>
        )}
        <div ref={bottomRef} />
      </div>
      <div className="chat-input-row">
        <textarea
          className="chat-input"
          rows={2}
          placeholder="Message agent… (Enter to send, Shift+Enter for newline)"
          value={input}
          onChange={e => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
        />
        <Button variant="primary" onClick={() => void handleSend()} disabled={isPending}>
          Send
        </Button>
      </div>
    </div>
  )
}
