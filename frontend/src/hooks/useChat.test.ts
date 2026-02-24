import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { renderHook, act, waitFor } from '@testing-library/react';
import { useChat } from './useChat';
import * as api from '../utils/api';

// Mock the startChatStream function
vi.mock('../utils/api');

describe('useChat', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe('Initial state', () => {
    it('should have initial state', () => {
      const { result } = renderHook(() => useChat('test-token'));

      expect(result.current.messages).toEqual([]);
      expect(result.current.isStreaming).toBe(false);
      expect(result.current.currentResponse).toBe('');
    });
  });

  describe('sendMessage', () => {
    it('should add user message and start streaming', async () => {
      vi.mocked(api.startChatStream).mockImplementation(async (request, onEvent) => {
        onEvent({ type: 'text', content: 'Hello from agent' });
        return () => { };
      });

      const { result } = renderHook(() => useChat('test-token'));
      await act(async () => {
        result.current.sendMessage('test message');
      });
      expect(api.startChatStream).toHaveBeenCalledWith(
        { message: 'test message' },
        expect.any(Function),
        'test-token')
      expect(result.current.isStreaming).toBeTruthy();
    });

    it('should handle text events and update currentResponse', async () => {
      // Simulate startChatStream calling onEvent with { type: 'text', content: '...' }
      vi.mocked(api.startChatStream).mockImplementation(async (request, onEvent) => {
        onEvent({ type: 'text', content: 'Hello from agent' });
        return () => { };
      });

      const { result } = renderHook(() => useChat('test-token'));
      await act(async () => {
        result.current.sendMessage('test message');
      });

      expect(result.current.currentResponse).toBe('Hello from agent');
    });

    it('should handle done event and save session', async () => {
      // Simulate done event, verify sessionId saved and assistant message added
      vi.mocked(api.startChatStream).mockImplementation(async (request, onEvent) => {
        onEvent({ type: 'text', content: 'Hello from agent' });
        onEvent({ type: 'done', session_id: 'testsessionid' });
        return () => { };
      });

      const { result } = renderHook(() => useChat('test-token'));
      await act(async () => {
        result.current.sendMessage('test message');
      });

      expect(result.current.messages).toHaveLength(2);
      expect(result.current.messages[0].content).toBe('test message');
      expect(result.current.messages[0].role).toBe('user');
      expect(result.current.messages[1].content).toBe('Hello from agent');
      expect(result.current.messages[1].role).toBe('assistant');
      expect(result.current.isStreaming).toBeFalsy();
    });

    it('should handle error events', async () => {
      vi.mocked(api.startChatStream).mockImplementation(async (request, onEvent) => {
        onEvent({ type: 'error', message: 'test error message' });
        return () => { };
      });

      const { result } = renderHook(() => useChat('test-token'));
      await act(async () => {
        result.current.sendMessage('test message');
      });

      expect(result.current.isStreaming).toBeFalsy();
    });

    it('should prevent double sending when streaming', async () => {
      // Call sendMessage twice quickly, verify startChatStream called only once
      vi.mocked(api.startChatStream).mockImplementation(async (request, onEvent) => {
        onEvent({ type: 'text', content: 'Hello from agent' });
        return () => { };
      });

      const { result } = renderHook(() => useChat('test-token'));
      await act(async () => {
        result.current.sendMessage('test message');
      });

      await act(async () => {
        result.current.sendMessage('second test message');
      });

      expect(api.startChatStream).toHaveBeenCalledExactlyOnceWith(
        { message: 'test message' },
        expect.any(Function),
        'test-token')
      expect(result.current.isStreaming).toBeTruthy();
    });

    it('should use session_id for subsequent messages', async () => {
      // Verify that session_id is persisted and used in the second message
      vi.mocked(api.startChatStream)
        .mockImplementationOnce(async (request, onEvent) => {
          // First message: no session_id yet
          expect(request.session_id).toBeUndefined();
          onEvent({ type: 'done', session_id: 'session-123' });
          return () => { };
        })
        .mockImplementationOnce(async (request, onEvent) => {
          // Second message: should include session_id
          expect(request.session_id).toBe('session-123');
          onEvent({ type: 'done', session_id: 'session-123' });
          return () => { };
        });

      const { result } = renderHook(() => useChat('test-token'));

      // First message
      await act(async () => {
        await result.current.sendMessage('First message');
      });

      // Second message
      await act(async () => {
        await result.current.sendMessage('Second message');
      });

      expect(api.startChatStream).toHaveBeenCalledTimes(2);
    });

    it('should accumulate multiple text events', async () => {
      // Verify that multiple text events are concatenated correctly
      vi.mocked(api.startChatStream).mockImplementation(async (request, onEvent) => {
        onEvent({ type: 'text', content: 'Hello ' });
        onEvent({ type: 'text', content: 'World' });
        onEvent({ type: 'text', content: '!' });
        onEvent({ type: 'done', session_id: 'test' });
        return () => { };
      });

      const { result } = renderHook(() => useChat('test-token'));

      await act(async () => {
        await result.current.sendMessage('test');
      });

      // Verify accumulated text in assistant message
      expect(result.current.messages).toHaveLength(2);
      expect(result.current.messages[1].content).toBe('Hello World!');
      expect(result.current.messages[1].role).toBe('assistant');
    });
  });
});
