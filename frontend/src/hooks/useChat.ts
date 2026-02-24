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
 * @returns {Object} Chat state and control functions
 * @returns {Message[]} messages - Array of confirmed chat messages (user and assistant)
 * @returns {string} currentResponse - Accumulated text of the currently streaming assistant response
 * @returns {boolean} isStreaming - Flag indicating if a response is currently being streamed
 * @returns {Function} sendMessage - Function to send a message and start streaming the response
 *
 * @example
 * const { messages, currentResponse, isStreaming, sendMessage } = useChat(token);
 *
 * // Send a message
 * await sendMessage("Hello!");
 *
 * // Display messages
 * messages.map(msg => <div>{msg.content}</div>)
 *
 * // Show streaming response
 * {isStreaming && <div>{currentResponse}</div>}
 */
export function useChat(token: string) {
  // Confirmed message history (both user and assistant messages)
  const [messages, setMessages] = useState<Message[]>([]);

  // Session ID for conversation continuity (managed internally, not exposed)
  // TODO: Consider adding startNewConversation() function to reset sessionId and messages
  const [sessionId, setSessionId] = useState<string | null>(null);

  // Flag to prevent concurrent message sending
  const [isStreaming, setIsStreaming] = useState(false);

  // Buffer for accumulating streamed assistant response (text events)
  const [currentResponse, setCurrentResponse] = useState('');

  const sendMessage = async (content: string) => {
    if (isStreaming) return;

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
          break;

        default:
          console.error(`Invalid event type has arrived. Ignoring.`);
          break;
      }
    }, token);

  };

  return { messages, currentResponse, isStreaming, sendMessage };
}
