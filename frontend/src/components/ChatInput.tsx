import { useState, FormEvent, KeyboardEvent } from 'react';

// TODO(human): Update the ChatInputProps interface and ChatInput component:
// 1. Change onSend prop type: (message: string, researchMode: boolean) => void
//    → (message: string) => void
// 2. Remove the isResearchMode state and its useState import (if no longer needed)
// 3. Remove the research toggle <button> (🔍) from JSX
// 4. Change onSend call: onSend(message, isResearchMode) → onSend(message)
// 5. Change placeholder to always be "メッセージを入力..."

interface ChatInputProps {
  onSend: (message: string) => void;
  onStop: () => void;
  disabled: boolean;
  isStreaming: boolean;
}

export function ChatInput({ onSend, onStop, disabled, isStreaming }: ChatInputProps) {
  const [input, setInput] = useState('');

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    const message = input.trim();
    if (!message || disabled) return;
    onSend(message);
    setInput('');
  };

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSubmit(e as any);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="chat-input">
      <input
        type="text"
        value={input}
        onChange={(e) => setInput(e.target.value)}
        onKeyDown={handleKeyDown}
        placeholder={"メッセージを入力..."}
        disabled={disabled}
      />
      {
        isStreaming
          ? <button type="button" onClick={onStop}>停止</button>
          : <button type="submit" disabled={disabled || !input.trim()}>送信</button>
      }

    </form>
  );
}
