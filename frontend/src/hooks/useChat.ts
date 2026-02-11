import { useState } from 'react';
import { startChatStream } from '../utils/api';
import type { Message } from '../types';

export function useChat() {
  // State定義（AIが提供）
  const [messages, setMessages] = useState<Message[]>([]);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [isStreaming, setIsStreaming] = useState(false);
  const [currentResponse, setCurrentResponse] = useState('');

  /**
   * メッセージを送信する関数
   *
   * TODO(human): この関数を実装してください
   *
   * 実装のヒント:
   * 1. isStreamingがtrueなら早期リターン（二重送信防止）
   * 2. ユーザーメッセージをmessagesに追加（setMessages）
   * 3. isStreamingをtrueに設定
   * 4. startChatStream()を呼び出し
   * 5. イベントハンドラで以下を処理:
   *    - 'text': currentResponseを更新（ストリーミング表示）
   *    - 'done': sessionIdを保存、アシスタントメッセージを追加、isStreaming=false
   *    - 'error': エラーログ、isStreaming=false
   *
   * 完全な実装例は ~/.claude/plans/concurrent-churning-finch.md にあります。
   */
  const sendMessage = async (content: string) => {
    // TODO(human): isStreamingチェック

    // TODO(human): ユーザーメッセージを追加
    // const userMessage: Message = { role: 'user', content, timestamp: Date.now() };
    // setMessages((prev) => [...prev, userMessage]);

    // TODO(human): isStreamingをtrueに、currentResponseをクリア

    // TODO(human): accumulatedTextを初期化

    // TODO(human): startChatStream()を呼び出し、イベント処理

    throw new Error('Not implemented');
  };

  return { messages, currentResponse, isStreaming, sendMessage };
}
