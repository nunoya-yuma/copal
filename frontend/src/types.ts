// APIリクエスト型（Rust側のChatRequestと対応）
export interface ChatRequest {
  session_id?: string;
  message: string;
}

// SSEイベント型（Rust側のSseEventDataと対応）
// IMPORTANT: session_id は snake_case（Rust側のserdeがsnake_caseを使用）
export type SseEvent =
  | { type: 'text'; content: string }
  | { type: 'done'; session_id: string }
  | { type: 'error'; message: string };

// UIメッセージ型
export interface Message {
  role: 'user' | 'assistant';
  content: string;
  timestamp: number;
}
