/**
 * Custom hook for managing chat functionality with SSE-based streaming.
 * Handles message state, session management, and real-time streaming responses.
 */
import { useState } from 'react';
import { startChatStream } from '../utils/api';
import type { ChatRequest, Message } from '../types';

/**
 * Custom hook for chat state management and message streaming.
 *
 * @param token - Bearer token for API authentication
 * @param onAuthError - Optional callback invoked when the server returns 401.
 *                      Typically used to clear the token and return to the login screen.
 * @returns {Object} Chat state and control functions
 * @returns {Message[]} messages - Array of confirmed chat messages (user and assistant)
 * @returns {string} currentResponse - Accumulated text of the currently streaming assistant response
 * @returns {boolean} isStreaming - Flag indicating if a response is currently being streamed
 * @returns {string | null} errorMessage - User-facing error message for non-auth errors, or null
 * @returns {Function} sendMessage - Function to send a message and start streaming the response
 *
 * @example
 * const { messages, currentResponse, isStreaming, errorMessage, sendMessage } = useChat(token, clearToken);
 *
 * // Send a message
 * await sendMessage("Hello!");
 *
 * // Display messages
 * messages.map(msg => <div>{msg.content}</div>)
 *
 * // Show streaming response
 * {isStreaming && <div>{currentResponse}</div>}
 *
 * // Show error message
 * {errorMessage && <div>{errorMessage}</div>}
 */
export function useChat(token: string, onAuthError?: () => void) {
  // Confirmed message history (both user and assistant messages)
  const [messages, setMessages] = useState<Message[]>([]);

  // Session ID for conversation continuity (managed internally, not exposed)
  // TODO: Consider adding startNewConversation() function to reset sessionId and messages
  const [sessionId, setSessionId] = useState<string | null>(null);

  // Flag to prevent concurrent message sending
  const [isStreaming, setIsStreaming] = useState(false);

  // Buffer for accumulating streamed assistant response (text events)
  const [currentResponse, setCurrentResponse] = useState('');

  // Error message to display in the UI (null when no error)
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const sendMessage = async (content: string) => {
    if (isStreaming) return;
    setErrorMessage(null);

    const userMessage: Message = { role: 'user', content, timestamp: Date.now() };
    setMessages((prev) => [...prev, userMessage]);

    setIsStreaming(true);
    setCurrentResponse('');

    let accumulatedText = '';
    let request: ChatRequest = {
      ...(sessionId && { session_id: sessionId }),
      message: userMessage.content
    };
    const cleanup = await startChatStream(request, (event) => {
      switch (event.type) {
        case 'text':
          setCurrentResponse((prev) => prev + event.content);
          accumulatedText += event.content;
          break;

        case 'done':
          const assistantMessage: Message = { role: 'assistant', content: accumulatedText, timestamp: Date.now() };
          setMessages((prev) => [...prev, assistantMessage]);
          setSessionId(event.session_id);
          setIsStreaming(false);
          setCurrentResponse('');
          break;

        case 'error':
          console.error(`Error event has been received. ${event.message}`);
          setIsStreaming(false);
          if (event.message.includes("401")) {
            onAuthError?.();
          }
          else {
            setErrorMessage("Unexpected error has occurred.");
          }
          break;

        default:
          console.error(`Invalid event type has arrived. Ignoring.`);
          break;
      }
    }, token);

  };

  return { messages, currentResponse, isStreaming, errorMessage, sendMessage };
}
