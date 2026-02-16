import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { ChatMessage } from './ChatMessage';
import type { Message } from '../types';

describe('ChatMessage', () => {
  describe('User messages', () => {
    it('should render user message as plain text', () => {
      const message: Message = { role: 'user', content: 'Hello', timestamp: Date.now() };

      render(<ChatMessage message={message} />);

      expect(screen.getByText('Hello')).toBeInTheDocument();
    });

    it('should apply correct CSS class for user role', () => {
      const message: Message = { role: 'user', content: 'Hello', timestamp: Date.now() };

      render(<ChatMessage message={message} />);

      const element = screen.getByText('Hello');
      const messageDiv = element.closest('.message');
      expect(messageDiv).toHaveClass('user');
    });
  });

  describe('Assistant messages', () => {
    it('should render assistant message with Markdown', () => {
      const message: Message = { role: 'assistant', content: '**bold** text', timestamp: Date.now() };

      render(<ChatMessage message={message} />);

      expect(screen.getByText('bold')).toBeInTheDocument();
      const boldElement = screen.getByText('bold');
      expect(boldElement.tagName).toBe('STRONG');
    });

    it('should apply correct CSS class for assistant role', () => {
      const message: Message = { role: 'assistant', content: 'Hello', timestamp: Date.now() };

      render(<ChatMessage message={message} />);

      const element = screen.getByText('Hello');
      const messageDiv = element.closest('.message');
      expect(messageDiv).toHaveClass('assistant');
    });

    it('should render code blocks', () => {
      const message: Message = { role: 'assistant', content: '```js\nconst x = 1;\n```', timestamp: Date.now() };

      render(<ChatMessage message={message} />);

      // Verify code block is rendered (syntax highlighting verified manually in browser)
      const preElement = screen.getByText('const x = 1;').closest('pre');
      expect(preElement).toBeInTheDocument();
    });
  });
});
