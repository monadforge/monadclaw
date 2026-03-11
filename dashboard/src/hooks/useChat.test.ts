import { renderHook, act, waitFor } from '@testing-library/react'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import { useChat } from './useChat'
import { createWrapper } from '../test/utils'

/** Build a ReadableStream that yields each string chunk in sequence. */
function makeStream(chunks: string[]): ReadableStream<Uint8Array> {
  let i = 0
  return new ReadableStream({
    pull(controller) {
      if (i < chunks.length) {
        controller.enqueue(new TextEncoder().encode(chunks[i++]))
      } else {
        controller.close()
      }
    },
  })
}

describe('useChat streaming', () => {
  beforeEach(() => {
    vi.resetAllMocks()
  })

  it('appends delta tokens to assistant message', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue({
        ok: true,
        body: makeStream([
          'data: Hello\n\n',
          'data:  world\n\n',
          'data: [DONE]\n\n',
        ]),
      }),
    )

    const { result } = renderHook(() => useChat(), { wrapper: createWrapper() })

    act(() => {
      result.current.send('hi')
    })

    await waitFor(() => {
      const assistant = result.current.messages.find(m => m.role === 'assistant')
      expect(assistant?.content).toBe('Hello world')
    })

    expect(result.current.isPending).toBe(false)
  })

  it('adds user message immediately', () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue({
        ok: true,
        body: makeStream(['data: [DONE]\n\n']),
      }),
    )

    const { result } = renderHook(() => useChat(), { wrapper: createWrapper() })

    act(() => {
      result.current.send('hello')
    })

    expect(result.current.messages[0]).toMatchObject({ role: 'user', content: 'hello' })
  })
})
