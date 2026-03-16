import { BrowserRouter, Routes, Route } from 'react-router-dom'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { Shell } from './components/layout/Shell'
import { AuthGuard } from './components/AuthGuard'
import LoginPage     from './pages/LoginPage'
import OverviewPage  from './pages/OverviewPage'
import ChatPage      from './pages/ChatPage'
import ChannelsPage  from './pages/ChannelsPage'
import AgentsPage    from './pages/AgentsPage'
import ConfigPage    from './pages/ConfigPage'
import SessionsPage  from './pages/SessionsPage'
import LogsPage      from './pages/LogsPage'
import UsagePage     from './pages/UsagePage'
import CronPage      from './pages/CronPage'
import SkillsPage    from './pages/SkillsPage'

const queryClient = new QueryClient()

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <Routes>
          <Route path="/login" element={<LoginPage />} />
          <Route element={<AuthGuard><Shell /></AuthGuard>}>
            <Route index           element={<OverviewPage />} />
            <Route path="chat"     element={<ChatPage />} />
            <Route path="channels" element={<ChannelsPage />} />
            <Route path="agents"   element={<AgentsPage />} />
            <Route path="config"   element={<ConfigPage />} />
            <Route path="sessions" element={<SessionsPage />} />
            <Route path="logs"     element={<LogsPage />} />
            <Route path="usage"    element={<UsagePage />} />
            <Route path="cron"     element={<CronPage />} />
            <Route path="skills"   element={<SkillsPage />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </QueryClientProvider>
  )
}
