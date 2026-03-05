import { useState } from 'react';

interface ChatInputProps {
  onSend: (message: string) => void;
  onStop: () => void;
  disabled: boolean;
  isStreaming: boolean;
}

export function ChatInput({ onSend, onStop, disabled, isStreaming }: ChatInputProps) {
  const [input, setInput] = useState('');

  const handleSubmit = () => {
    const message = input.trim();
    if (!message || disabled) return;
    onSend(message);
    setInput('');
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  };

  return (
    <form onSubmit={(e) => { e.preventDefault(); handleSubmit(); }} className="chat-input">
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
