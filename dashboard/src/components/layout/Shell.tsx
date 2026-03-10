import { Outlet } from 'react-router-dom'
import { Topbar } from './Topbar'
import { Sidebar } from './Sidebar'
import { useUiStore } from '../../store/uiStore'
import '../../styles/layout.css'

export function Shell() {
  const { sidebarCollapsed, toggleSidebar } = useUiStore()
  return (
    <div className={`shell${sidebarCollapsed ? ' collapsed' : ''}`}>
      <Topbar />
      <Sidebar collapsed={sidebarCollapsed} onToggle={toggleSidebar} />
      <main className="content">
        <Outlet />
      </main>
    </div>
  )
}
