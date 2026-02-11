import { useEffect, useRef } from 'react';
import { ChatMessage } from './ChatMessage';
import type { Message } from '../types';

interface ChatContainerProps {
  messages: Message[];
  currentResponse: string;
  isStreaming: boolean;
}

export function ChatContainer({
  messages,
  currentResponse,
  isStreaming,
}: ChatContainerProps) {
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (containerRef.current) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight;
    }
  }, [messages, currentResponse]);

  return (
    <div ref={containerRef} className="chat-container">
      {messages.map((message, index) => (
        <ChatMessage key={index} message={message} />
      ))}
      {isStreaming && currentResponse && (
        <ChatMessage
          message={{
            role: 'assistant',
            content: currentResponse,
            timestamp: Date.now(),
          }}
        />
      )}
    </div>
  );
}
