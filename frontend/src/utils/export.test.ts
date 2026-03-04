import { describe, it, expect } from 'vitest';
import { exportToMarkdown } from './export';
import type { Message } from '../types';

const makeMessage = (role: 'user' | 'assistant', content: string): Message => ({
  role,
  content,
  timestamp: 0,
});

describe('exportToMarkdown', () => {
  it('should start with a title header', () => {
    const result = exportToMarkdown([makeMessage('user', 'hello')]);
    expect(result).toMatch(/^# Copal Session/);
  });

  it('should include user message under "## User" heading', () => {
    const result = exportToMarkdown([makeMessage('user', 'What is quantum computing?')]);
    expect(result).toContain('## User\nWhat is quantum computing?');
  });

  it('should include assistant message under "## Assistant" heading', () => {
    const result = exportToMarkdown([makeMessage('assistant', 'It is a type of computing...')]);
    expect(result).toContain('## Assistant\nIt is a type of computing...');
  });

  it('should render multiple messages in order', () => {
    const messages = [
      makeMessage('user', 'first'),
      makeMessage('assistant', 'second'),
      makeMessage('user', 'third'),
    ];
    const result = exportToMarkdown(messages);
    const userPos = result.indexOf('first');
    const assistantPos = result.indexOf('second');
    const user2Pos = result.indexOf('third');
    expect(userPos).toBeLessThan(assistantPos);
    expect(assistantPos).toBeLessThan(user2Pos);
  });

  it('should return just the header for empty messages array', () => {
    const result = exportToMarkdown([]);
    expect(result.trim()).toBe('# Copal Session');
  });

  it('should separate messages with blank lines', () => {
    const result = exportToMarkdown([
      makeMessage('user', 'hello'),
      makeMessage('assistant', 'hi'),
    ]);
    expect(result).toBe('# Copal Session\n\n## User\nhello\n\n## Assistant\nhi');
  });
});
