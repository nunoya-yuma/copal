import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { ChatInput } from './ChatInput';

describe('ChatInput', () => {
  describe('Send button (not streaming)', () => {
    it('should show Send button when isStreaming is false', () => {
      render(<ChatInput onSend={() => { }} onStop={() => { }} disabled={false} isStreaming={false} />)

      expect(screen.getByText('送信')).toBeInTheDocument()
      expect(screen.queryByText('停止')).not.toBeInTheDocument()
    });

    it('should be disabled when input is empty', () => {
      render(<ChatInput onSend={() => { }} onStop={() => { }} disabled={false} isStreaming={false} />)

      expect(screen.getByText('送信')).toBeDisabled()
    });

    it('should call onSend with the typed message', async () => {
      const onSend = vi.fn();
      render(<ChatInput onSend={onSend} onStop={() => { }} disabled={false} isStreaming={false} />)

      await userEvent.type(screen.getByRole('textbox'), 'Hello World');
      await userEvent.click(screen.getByText('送信'));

      expect(onSend).toHaveBeenCalledWith('Hello World');
    });

    it('should clear input after sending', async () => {
      render(<ChatInput onSend={() => { }} onStop={() => { }} disabled={false} isStreaming={false} />)
      const sut = screen.getByRole('textbox');

      await userEvent.type(sut, 'Hello World');
      await userEvent.click(screen.getByText('送信'));

      expect(sut).toHaveValue('');
    });

    it('should send message when Enter key is pressed', async () => {
      const onSend = vi.fn();
      render(<ChatInput onSend={onSend} onStop={() => { }} disabled={false} isStreaming={false} />)

      await userEvent.type(screen.getByRole('textbox'), 'Hello World{Enter}');

      expect(onSend).toHaveBeenCalledWith('Hello World');
    });
  });

  describe('Stop button (streaming)', () => {
    it('should show Stop button when isStreaming is true', () => {
      render(<ChatInput onSend={() => { }} onStop={() => { }} disabled={false} isStreaming={true} />)

      expect(screen.getByText('停止')).toBeInTheDocument()
      expect(screen.queryByText('送信')).not.toBeInTheDocument()
    });

    it('should call onStop when Stop button is clicked', async () => {
      const onStop = vi.fn();
      render(<ChatInput onSend={() => { }} onStop={onStop} disabled={true} isStreaming={true} />)
      const element = screen.getByText('停止');

      await userEvent.click(element);

      expect(onStop).toHaveBeenCalledOnce()
    });
  });
});
