import { render, screen } from '@testing-library/react'
import { QueryClientProvider, QueryClient } from '@tanstack/react-query'
import OverviewPage from '../../pages/OverviewPage'

vi.mock('../../api/status', () => ({
  fetchStatus: vi.fn().mockResolvedValue({
    status: 'online',
    provider: 'openai',
    model: 'gpt-4o',
    memoryShortTerm: 12,
    memoryLongTerm: 347,
    uptimeSeconds: 3600,
    messagesToday: 5,
  }),
}))

test('renders status cards with data', async () => {
  const client = new QueryClient({ defaultOptions: { queries: { retry: false } } })
  render(
    <QueryClientProvider client={client}>
      <OverviewPage />
    </QueryClientProvider>
  )
  expect(await screen.findByText('online')).toBeInTheDocument()
  expect(await screen.findByText('gpt-4o')).toBeInTheDocument()
})
