import type { ChatRequest, SseEvent } from '../types';

/**
 * Start an SSE (Server-Sent Events) stream to the chat API and handle incoming events.
 *
 * This function initiates a POST request to `/api/chat` and processes the response as a stream
 * of SSE events. Events are delivered via the `onEvent` callback as they arrive.
 *
 * @param request - The chat request containing the message and optional session_id
 * @param onEvent - Callback function invoked for each SSE event (text/done/error)
 * @returns A Promise resolving to a cleanup function that cancels the stream when called
 *
 * @example
 * ```typescript
 * const cleanup = await startChatStream(
 *   { message: 'Hello', session_id: 'abc-123' },
 *   (event) => {
 *     if (event.type === 'text') {
 *       console.log('Received:', event.content);
 *     } else if (event.type === 'done') {
 *       console.log('Stream completed. Session:', event.session_id);
 *     } else if (event.type === 'error') {
 *       console.error('Error:', event.message);
 *     }
 *   }
 * );
 *
 * // Later, to cancel the stream:
 * cleanup();
 * ```
 */
export async function startChatStream(
  request: ChatRequest,
  onEvent: (event: SseEvent) => void
): Promise<() => void> {
  let response;
  try {
    response = await fetch('/api/chat', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    });
  } catch (error) {
    onEvent({ type: "error", message: `Network error: ${error}` });
    return () => { };
  }

  if (!response.ok) {
    onEvent({ type: "error", message: `Http post error: ${response.status}` });
    return () => { };
  }
  if (!response.body) {
    throw new Error(`Readable stream is not supplied: ${response.status}`);
  }

  const reader = response.body.getReader();
  const decoder = new TextDecoder();
  let buffer = '';

  // TODO: Refactor - Extract readLoop as a separate async function and run in background
  // This would allow startChatStream to return immediately while continuing to read the stream.
  // Note: Requires updating tests to handle async timing (e.g., setTimeout or Promise resolution)
  while (true) {
    const { done, value } = await reader.read();
    if (done) {
      break;
    }

    buffer += decoder.decode(value, { stream: true })
    const lines = buffer.split('\n');
    buffer = lines.pop() || '';

    for (const line of lines) {
      if (line.startsWith('data: ')) {
        const data = line.slice(6);
        let event;
        try {
          event = JSON.parse(data);
        } catch (error) {
          onEvent({ type: "error", message: `Json error: ${error}` });
          return () => { };
        }
        onEvent(event);
      }
    }
  }

  return () => reader.cancel();
}
