import React, { useState } from 'react';

interface TokenGateProps {
  onSubmit: (token: string) => void;
}

export function TokenGate({ onSubmit }: TokenGateProps) {
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    const token = input.trim();
    if (!token) return;

    setIsLoading(true);
    setError(null);

    try {
      const response = await fetch('/api/verify', { headers: { Authorization: `Bearer ${token}` } });
      if (response.ok) {
        onSubmit(token);
      } else {
        setError('トークンが無効です。');
      }
    } catch (e) {
      console.error('Token verification failed:', e);
      setError('トークンの送信に失敗しました。');
    }
    finally {
      setIsLoading(false);
    }
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
        {error && <p className="token-gate-error">{error}</p>}
        <button type="submit" disabled={!input.trim() || isLoading}>
          {isLoading ? '確認中...' : '接続'}
        </button>
      </form>
    </div>
  );
}
