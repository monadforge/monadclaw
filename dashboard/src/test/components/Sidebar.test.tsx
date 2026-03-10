import { render, screen, fireEvent } from '@testing-library/react'
import { MemoryRouter } from 'react-router-dom'
import { Sidebar } from '../../components/layout/Sidebar'

const wrap = (ui: React.ReactElement) =>
  render(<MemoryRouter>{ui}</MemoryRouter>)

test('renders all nav group labels', () => {
  wrap(<Sidebar collapsed={false} onToggle={() => {}} />)
  expect(screen.getByText('MONITOR')).toBeInTheDocument()
  expect(screen.getByText('MANAGE')).toBeInTheDocument()
  expect(screen.getByText('SETTINGS')).toBeInTheDocument()
})

test('renders Overview link', () => {
  wrap(<Sidebar collapsed={false} onToggle={() => {}} />)
  expect(screen.getByRole('link', { name: /overview/i })).toBeInTheDocument()
})

test('collapse toggle fires callback', () => {
  const onToggle = vi.fn()
  wrap(<Sidebar collapsed={false} onToggle={onToggle} />)
  fireEvent.click(screen.getByRole('button', { name: /collapse/i }))
  expect(onToggle).toHaveBeenCalledOnce()
})
