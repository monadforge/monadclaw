import { useEffect, useState } from 'react'
import { Navigate } from 'react-router-dom'
import { getAuthStatus } from '../api/auth'
import { useAuthStore } from '../store/authStore'

export function AuthGuard({ children }: { children: React.ReactNode }) {
  const token = useAuthStore((s) => s.token)
  const [checking, setChecking] = useState(true)
  const [isProtected, setIsProtected] = useState(false)

  useEffect(() => {
    getAuthStatus()
      .then(({ protected: p }) => setIsProtected(p))
      .catch(() => setIsProtected(false))
      .finally(() => setChecking(false))
  }, [])

  if (checking) return null

  if (isProtected && !token) {
    return <Navigate to="/login" replace />
  }

  return <>{children}</>
}
