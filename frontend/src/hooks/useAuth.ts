import { useState } from 'react';

const STORAGE_KEY = 'copal_api_token';

export interface UseAuthReturn {
  token: string;
  setToken: (token: string) => void;
  clearToken: () => void;
  isAuthenticated: boolean;
}

/**
 * Custom hook for managing API token authentication.
 *
 * Persists the token in localStorage so users don't need to re-enter it on refresh.
 *
 * @returns {UseAuthReturn} Auth state and control functions
 */
export function useAuth(): UseAuthReturn {
  const [token, setCurrentToken] = useState(() => localStorage.getItem(STORAGE_KEY) ?? '');
  const isAuthenticated = token.length > 0;

  function setToken(token: string) {
    localStorage.setItem(STORAGE_KEY, token);
    setCurrentToken(token);
  }

  function clearToken() {
    localStorage.removeItem(STORAGE_KEY);
    setCurrentToken('');
  }

  return { token, setToken, clearToken, isAuthenticated };
}
