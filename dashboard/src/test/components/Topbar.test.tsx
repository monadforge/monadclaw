import { render, screen } from '@testing-library/react'
import { Topbar } from '../../components/layout/Topbar'

test('renders monadclaw logo', () => {
  render(<Topbar />)
  expect(screen.getByText('[monadclaw]')).toBeInTheDocument()
})

test('renders status pill', () => {
  render(<Topbar />)
  expect(screen.getByRole('status')).toBeInTheDocument()
})
