import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { startChatStream } from './api';
import type { ChatRequest, SseEvent } from '../types';

describe('startChatStream', () => {
  beforeEach(() => {
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe('Normal case', () => {
    it('should stream SSE events and call onEvent for each event', async () => {
      // Mock a ReadableStream that emits two SSE events
      const mockResponse = {
        ok: true,
        body: new ReadableStream({
          start(controller) {
            // Simulate server sending two complete SSE events
            controller.enqueue(new TextEncoder().encode('data: {"type":"text","content":"Hello"}\n'));
            controller.enqueue(new TextEncoder().encode('data: {"type":"done","session_id":"123"}\n'));
            controller.close();
          }
        })
      };

      (global.fetch as any).mockResolvedValue(mockResponse);

      const receivedEvents: SseEvent[] = [];

      const request: ChatRequest = { message: 'test' };
      await startChatStream(request, (event) => {
        receivedEvents.push(event);
      });

      expect(receivedEvents).toHaveLength(2);
      expect(receivedEvents[0]).toEqual({ type: 'text', content: 'Hello' });
      expect(receivedEvents[1]).toEqual({ type: 'done', session_id: '123' });
    });

    it('should handle incomplete lines correctly (buffering)', async () => {
      // Test the buffering logic: SSE data can arrive in chunks at arbitrary boundaries
      // This simulates a single JSON object split across two chunks
      const mockResponse = {
        ok: true,
        body: new ReadableStream({
          start(controller) {
            // First chunk: incomplete JSON (missing closing brace and newline)
            controller.enqueue(new TextEncoder().encode('data: {"type":"te'));
            // Second chunk: completes the JSON
            controller.enqueue(new TextEncoder().encode('xt","content":"Hello"}\n'));
            controller.close();
          }
        })
      };

      (global.fetch as any).mockResolvedValue(mockResponse);

      const receivedEvents: SseEvent[] = [];

      const request: ChatRequest = { message: 'test' };
      await startChatStream(request, (event) => {
        receivedEvents.push(event);
      });

      expect(receivedEvents).toHaveLength(1);
      expect(receivedEvents[0]).toEqual({ type: 'text', content: 'Hello' });
    });
  })

  describe('Error handling', () => {
    it('should handle network errors', async () => {
      // Simulate fetch throwing an error (e.g., network unavailable, DNS failure)
      (global.fetch as any).mockRejectedValue(new Error('connection error'));

      const receivedEvents: SseEvent[] = [];
      const request: ChatRequest = { message: 'test' };
      await startChatStream(request, (event) => {
        receivedEvents.push(event);
      });

      expect(receivedEvents).toHaveLength(1);
      expect(receivedEvents[0]).toEqual({ type: 'error', message: 'Network error: Error: connection error' });
    });

    it('should handle Http response errors', async () => {
      // Simulate HTTP error response (4xx or 5xx status codes)
      const mockResponse = {
        ok: false,
        status: 404,
      };

      (global.fetch as any).mockResolvedValue(mockResponse);

      const receivedEvents: SseEvent[] = [];
      const request: ChatRequest = { message: 'test' };
      await startChatStream(request, (event) => {
        receivedEvents.push(event);
      });

      expect(receivedEvents).toHaveLength(1);
      expect(receivedEvents[0]).toEqual({ type: 'error', message: 'Http post error: 404' });
    });

    it('should handle JSON parse errors', async () => {
      // Test handling of malformed JSON in SSE data field
      const mockResponse = {
        ok: true,
        body: new ReadableStream({
          start(controller) {
            // Send syntactically invalid JSON (missing quotes, etc.)
            controller.enqueue(new TextEncoder().encode('data: {invalid json}}\n'));
            controller.close();
          }
        })
      };
      (global.fetch as any).mockResolvedValue(mockResponse);

      const receivedEvents: SseEvent[] = [];
      const request: ChatRequest = { message: 'test' };
      await startChatStream(request, (event) => {
        receivedEvents.push(event);
      });

      expect(receivedEvents).toHaveLength(1);
      expect(receivedEvents[0].type).toBe('error');
      expect(receivedEvents[0].message).toContain('Json error');
      expect(receivedEvents[0].message).toContain('SyntaxError');
    });
  })

  describe('Clean up', () => {
    it('should return a cleanup function that cancels the reader', async () => {
      // Create a mock cancel function to verify it's called
      const mockCancel = vi.fn();

      // Mock ReadableStream with manual getReader() to inject our mock cancel
      const mockResponse = {
        ok: true,
        body: {
          getReader: () => ({
            read: vi.fn().mockResolvedValue({ done: true, value: undefined }),
            cancel: mockCancel  // Our spy function
          })
        }
      };
      (global.fetch as any).mockResolvedValue(mockResponse);

      const request: ChatRequest = { message: 'test' };
      const cleanup = await startChatStream(request, () => { });

      // Call the returned cleanup function
      cleanup();

      // Verify that reader.cancel() was invoked
      expect(mockCancel).toHaveBeenCalled();
    });
  })
});
