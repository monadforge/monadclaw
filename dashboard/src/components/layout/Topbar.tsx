import './Topbar.css'

type Status = 'online' | 'offline' | 'degraded' | 'unknown'

interface TopbarProps {
  status?: Status
}

export function Topbar({ status = 'unknown' }: TopbarProps) {
  return (
    <header className="topbar">
      <span className="topbar-logo">[monadclaw]</span>
      <div className="topbar-spacer" />
      <div className="status-pill" data-status={status} role="status">
        <span className="status-dot" />
        <span>{status}</span>
      </div>
    </header>
  )
}
