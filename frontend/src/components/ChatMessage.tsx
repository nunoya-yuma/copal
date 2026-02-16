import ReactMarkdown from 'react-markdown';
import type { Message } from '../types';

interface ChatMessageProps {
  message: Message;
}

/**
 * TODO(human): このコンポーネントを実装してください
 *
 * 実装のヒント:
 * 1. div要素を返す（className={`message ${message.role}`}）
 * 2. message.roleが'assistant'の場合:
 *    - ReactMarkdownコンポーネントを使用
 *    - componentsプロップでcodeブロックをカスタマイズ
 *    - SyntaxHighlighterでシンタックスハイライト
 * 3. message.roleが'user'の場合:
 *    - <p>{message.content}</p>でシンプルに表示
 *
 * 完全な実装例は ~/.claude/plans/concurrent-churning-finch.md にあります。
 */
export function ChatMessage({ message }: ChatMessageProps) {
  return (<div className={`message ${message.role}`}>
    {message.role === 'assistant' ? (
      <ReactMarkdown>{message.content}</ReactMarkdown>
    ) : (
      <p>{message.content}</p>
    )}
  </div>);
}
