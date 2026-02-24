import { useState, FormEvent } from 'react';

interface TokenGateProps {
  onSubmit: (token: string) => void;
}

export function TokenGate({ onSubmit }: TokenGateProps) {
  const [input, setInput] = useState('');

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    const token = input.trim();
    if (!token) return;
    onSubmit(token);
  };

  return (
    <div className="token-gate">
      <h1>Copal</h1>
      <p>API トークンを入力してください</p>
      <form onSubmit={handleSubmit} className="token-gate-form">
        <input
          type="password"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder="Bearer token"
          autoFocus
        />
        <button type="submit" disabled={!input.trim()}>
          接続
        </button>
      </form>
    </div>
  );
}
