import { apiFetch } from './client'

export interface AuthStatus {
  protected: boolean
}

export interface LoginResponse {
  token: string
}

export async function getAuthStatus(): Promise<AuthStatus> {
  return apiFetch<AuthStatus>('/api/v1/auth/status')
}

export async function login(password: string): Promise<LoginResponse> {
  return apiFetch<LoginResponse>('/api/v1/auth/login', {
    method: 'POST',
    body: JSON.stringify({ password }),
  })
}
