import { Outlet } from 'react-router-dom'
import { Topbar } from './Topbar'
import { Sidebar } from './Sidebar'
import { useUiStore } from '../../store/uiStore'
import { useStatus } from '../../hooks/useStatus'
import '../../styles/layout.css'

export function Shell() {
  const { sidebarCollapsed, toggleSidebar } = useUiStore()
  const { data } = useStatus()
  return (
    <div className={`shell${sidebarCollapsed ? ' collapsed' : ''}`}>
      <Topbar status={data?.status} />
      <Sidebar collapsed={sidebarCollapsed} onToggle={toggleSidebar} />
      <main className="content">
        <Outlet />
      </main>
    </div>
  )
}
