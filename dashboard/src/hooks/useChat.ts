import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { fetchHistory, sendMessage } from '../api/chat'
import type { ChatMessage } from '../types/api'

export function useChat() {
  const qc = useQueryClient()
  const { data: messages = [] } = useQuery<ChatMessage[]>({
    queryKey: ['chat-history'],
    queryFn: fetchHistory,
  })
  const mutation = useMutation({
    mutationFn: (content: string) => sendMessage(content),
    onSuccess: msg => {
      qc.setQueryData<ChatMessage[]>(['chat-history'], prev => [
        ...(prev ?? []),
        msg,
      ])
    },
  })
  return { messages, send: mutation.mutateAsync, isPending: mutation.isPending }
}
