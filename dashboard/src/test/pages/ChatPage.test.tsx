import { render, screen } from '@testing-library/react'
import { QueryClientProvider, QueryClient } from '@tanstack/react-query'
import ChatPage from '../../pages/ChatPage'

vi.mock('../../api/chat', () => ({
  fetchHistory: vi.fn().mockResolvedValue([]),
  sendMessage: vi.fn().mockResolvedValue({
    id: '1', role: 'assistant', content: 'Hi!', timestamp: new Date().toISOString(),
  }),
}))

const wrap = (ui: React.ReactElement) =>
  render(<QueryClientProvider client={new QueryClient()}>{ui}</QueryClientProvider>)

test('renders message input', async () => {
  wrap(<ChatPage />)
  expect(await screen.findByPlaceholderText(/message/i)).toBeInTheDocument()
})

test('send button is present', async () => {
  wrap(<ChatPage />)
  expect(await screen.findByRole('button', { name: /send/i })).toBeInTheDocument()
})
