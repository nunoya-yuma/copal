import ReactMarkdown from 'react-markdown';
import SyntaxHighlighter from 'react-syntax-highlighter/dist/esm/prism';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';
import type { Components } from 'react-markdown';
import type { Message } from '../types';

interface ChatMessageProps {
  message: Message;
}

const CodeBlock: Components['code'] = ({ className, children }) => {
  if (className?.includes('language-')) {
    const language = className?.replace('language-', '');
    return <SyntaxHighlighter style={vscDarkPlus} language={language} PreTag="div" >{String(children).replace(/\n$/, '')}</SyntaxHighlighter>;
  }

  return <code className={className}>{children}</code>;
};

export function ChatMessage({ message }: ChatMessageProps) {
  return (
    <div className={`message ${message.role}`}>
      {message.role === 'assistant' ? (
        <ReactMarkdown components={{ code: CodeBlock }}>
          {message.content}
        </ReactMarkdown>
      ) : (
        <p>{message.content}</p>
      )}
    </div>
  );
}
