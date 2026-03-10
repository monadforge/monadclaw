import { describe, test, expect, vi, beforeEach } from 'vitest'
import { apiFetch } from '../../api/client'

describe('apiFetch', () => {
  beforeEach(() => {
    vi.stubGlobal('fetch', vi.fn())
    localStorage.clear()
  })

  test('adds Authorization header when token present', async () => {
    const mockFetch = vi.mocked(fetch)
    mockFetch.mockResolvedValueOnce(
      new Response(JSON.stringify({ ok: true }), { status: 200 })
    )

    localStorage.setItem('monadclaw-token', 'test-token')
    await apiFetch('/api/v1/status')

    expect(mockFetch).toHaveBeenCalledWith(
      '/api/v1/status',
      expect.objectContaining({
        headers: expect.objectContaining({
          Authorization: 'Bearer test-token',
        }),
      })
    )
  })

  test('throws ApiError on non-2xx response', async () => {
    vi.mocked(fetch).mockResolvedValueOnce(
      new Response(
        JSON.stringify({ error: { code: 'NOT_FOUND', message: 'not found' } }),
        { status: 404 }
      )
    )

    await expect(apiFetch('/api/v1/missing')).rejects.toMatchObject({
      error: { code: 'NOT_FOUND' },
    })
  })
})
