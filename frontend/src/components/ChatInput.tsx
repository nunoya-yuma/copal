import { useState, FormEvent, KeyboardEvent } from 'react';

interface ChatInputProps {
  onSend: (message: string, researchMode: boolean) => void;
  onStop: () => void;
  disabled: boolean;
  isStreaming: boolean;
}

export function ChatInput({ onSend, onStop, disabled, isStreaming }: ChatInputProps) {
  const [input, setInput] = useState('');
  const [isResearchMode, setIsResearchMode] = useState(false);

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    const message = input.trim();
    if (!message || disabled) return;
    onSend(message, isResearchMode);
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
      <button
        type="button"
        className={`research-toggle ${isResearchMode ? 'active' : ''}`}
        onClick={() => setIsResearchMode((prev) => !prev)}
        title="リサーチモード"
        disabled={disabled}
      >
        🔍
      </button>
      <input
        type="text"
        value={input}
        onChange={(e) => setInput(e.target.value)}
        onKeyDown={handleKeyDown}
        placeholder={isResearchMode ? "調査トピックを入力..." : "メッセージを入力..."}
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
