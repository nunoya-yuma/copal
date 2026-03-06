import { useEffect, useRef } from 'react';
import { ChatMessage } from './ChatMessage';
import { ThinkingDots } from './ThinkingDots';
import type { Message } from '../types';

interface ChatContainerProps {
  messages: Message[];
  currentResponse: string;
  isStreaming: boolean;
  currentPhase: string | null;
}

export function ChatContainer({
  messages,
  currentResponse,
  isStreaming,
  currentPhase,
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
      {isStreaming && !currentResponse && !currentPhase && (
        <div className="thinking-waiting">
          <ThinkingDots />
        </div>
      )}
    </div>
  );
}
