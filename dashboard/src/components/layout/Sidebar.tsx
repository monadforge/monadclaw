import { NavLink } from 'react-router-dom'
import './Sidebar.css'

interface NavItem { label: string; path: string; icon: string }
interface NavGroup { label: string; items: NavItem[] }

const NAV: NavGroup[] = [
  {
    label: 'MONITOR',
    items: [
      { label: 'Overview', path: '/', icon: '◈' },
      { label: 'Logs',     path: '/logs', icon: '≡' },
      { label: 'Sessions', path: '/sessions', icon: '◷' },
      { label: 'Usage',    path: '/usage', icon: '◎' },
    ],
  },
  {
    label: 'MANAGE',
    items: [
      { label: 'Agents',   path: '/agents', icon: '◉' },
      { label: 'Channels', path: '/channels', icon: '◈' },
      { label: 'Skills',   path: '/skills', icon: '◆' },
      { label: 'Cron',     path: '/cron', icon: '◷' },
    ],
  },
  {
    label: 'SETTINGS',
    items: [
      { label: 'Config',   path: '/config', icon: '◧' },
    ],
  },
  {
    label: 'INTERACT',
    items: [
      { label: 'Chat',     path: '/chat', icon: '◌' },
    ],
  },
]

interface SidebarProps {
  collapsed: boolean
  onToggle: () => void
}

export function Sidebar({ collapsed, onToggle }: SidebarProps) {
  return (
    <nav className={`sidebar${collapsed ? ' collapsed' : ''}`}>
      <button
        className="sidebar-toggle"
        onClick={onToggle}
        aria-label="collapse sidebar"
      >
        {collapsed ? '›' : '‹'}
      </button>
      {NAV.map(group => (
        <div key={group.label} className="nav-group">
          {!collapsed && (
            <div className="nav-group-label">{group.label}</div>
          )}
          {group.items.map(item => (
            <NavLink
              key={item.path}
              to={item.path}
              end={item.path === '/'}
              className={({ isActive }) =>
                `nav-link${isActive ? ' active' : ''}`
              }
            >
              <span className="nav-icon">{item.icon}</span>
              {!collapsed && <span>{item.label}</span>}
            </NavLink>
          ))}
        </div>
      ))}
    </nav>
  )
}
