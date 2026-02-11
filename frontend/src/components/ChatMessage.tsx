import ReactMarkdown from 'react-markdown';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';
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
  // TODO(human): div要素でmessageをラップ

  // TODO(human): message.roleで条件分岐
  // - 'assistant': ReactMarkdownを使用
  // - 'user': pタグで表示

  return <div>Not implemented</div>;
}
