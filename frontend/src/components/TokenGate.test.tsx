import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { TokenGate } from './TokenGate';

// Helper: create a mock Response with a given status code
// (fetch doesn't throw on 4xx - it resolves, so we simulate that here)
const mockFetchResponse = (status: number) =>
    Promise.resolve(new Response(null, { status }));

describe('TokenGate', () => {
    beforeEach(() => {
        // Replace global fetch with a vitest mock function
        vi.stubGlobal('fetch', vi.fn());
    });

    afterEach(() => {
        // Restore original fetch after each test
        vi.unstubAllGlobals();
    });

    describe('Initial state', () => {
        it('should render input field and submit button', () => {
            render(<TokenGate onSubmit={vi.fn()} />);

            expect(screen.getByPlaceholderText('Bearer token')).toBeInTheDocument();
            expect(screen.getByRole('button', { name: '接続' })).toBeInTheDocument();
        });

        it('should disable submit button when input is empty', () => {
            render(<TokenGate onSubmit={vi.fn()} />);

            expect(screen.getByRole('button', { name: '接続' })).toBeDisabled();
        });

        it('should enable submit button when input has value', async () => {
            const user = userEvent.setup();
            render(<TokenGate onSubmit={vi.fn()} />);

            await user.type(screen.getByPlaceholderText('Bearer token'), 'some-token');

            expect(screen.getByRole('button', { name: '接続' })).toBeEnabled();
        });
    });

    describe('Form submission', () => {
        it('should call /api/verify with correct Bearer token header', async () => {
            vi.mocked(fetch).mockImplementation(() => mockFetchResponse(200));
            const user = userEvent.setup();
            render(<TokenGate onSubmit={vi.fn()} />);

            await user.type(screen.getByPlaceholderText('Bearer token'), 'my-secret-token');
            await user.click(screen.getByRole('button', { name: '接続' }));

            expect(fetch).toHaveBeenCalledWith('/api/verify', {
                headers: { Authorization: 'Bearer my-secret-token' },
            });
        });

        it('should call onSubmit with token when /api/verify returns 200', async () => {
            vi.mocked(fetch).mockImplementation(() => mockFetchResponse(200));
            const user = userEvent.setup();
            const onSubmit = vi.fn();
            render(<TokenGate onSubmit={onSubmit} />);

            await user.type(screen.getByPlaceholderText('Bearer token'), 'valid-token');
            await user.click(screen.getByRole('button', { name: '接続' }));

            expect(onSubmit).toHaveBeenCalledWith('valid-token');
        });

        it('should show error message when /api/verify returns 401', async () => {
            vi.mocked(fetch).mockImplementation(() => mockFetchResponse(401));
            const user = userEvent.setup();
            render(<TokenGate onSubmit={vi.fn()} />);

            await user.type(screen.getByPlaceholderText('Bearer token'), 'wrong-token');
            await user.click(screen.getByRole('button', { name: '接続' }));

            expect(screen.getByText('トークンが無効です。')).toBeInTheDocument();
        });

        it('should not call onSubmit when /api/verify returns 401', async () => {
            vi.mocked(fetch).mockImplementation(() => mockFetchResponse(401));
            const user = userEvent.setup();
            const onSubmit = vi.fn();
            render(<TokenGate onSubmit={onSubmit} />);

            await user.type(screen.getByPlaceholderText('Bearer token'), 'wrong-token');
            await user.click(screen.getByRole('button', { name: '接続' }));

            expect(onSubmit).not.toHaveBeenCalled();
        });

        it('should show error message on network error', async () => {
            vi.mocked(fetch).mockRejectedValue(new Error('Network error'));
            const user = userEvent.setup();
            render(<TokenGate onSubmit={vi.fn()} />);

            await user.type(screen.getByPlaceholderText('Bearer token'), 'any-token');
            await user.click(screen.getByRole('button', { name: '接続' }));

            // token-gate-error クラスの要素が表示されていることを確認
            // (テキストは実装に依存するため、クラスで検証)
            expect(document.querySelector('.token-gate-error')).toBeInTheDocument();
        });

        it('should show loading indicator and disable button while verifying', async () => {
            // Arrange: fetch that never resolves, keeping the component in loading state
            vi.mocked(fetch).mockReturnValue(new Promise(() => {}));
            const user = userEvent.setup();
            render(<TokenGate onSubmit={vi.fn()} />);
            await user.type(screen.getByPlaceholderText('Bearer token'), 'my-token');

            // Act: click submit (userEvent resolves after React flushes state updates,
            // but before the pending fetch resolves)
            await user.click(screen.getByRole('button', { name: '接続' }));

            // Assert: button reflects loading state
            const sut = screen.getByRole('button', { name: '確認中...' });
            expect(sut).toBeInTheDocument();
            expect(sut).toBeDisabled();
        });

        it('should clear previous error message when submitting again', async () => {
            // Arrange: first attempt fails with 401
            vi.mocked(fetch).mockImplementation(() => mockFetchResponse(401));
            const user = userEvent.setup();
            render(<TokenGate onSubmit={vi.fn()} />);
            await user.type(screen.getByPlaceholderText('Bearer token'), 'wrong-token');
            await user.click(screen.getByRole('button', { name: '接続' }));
            expect(screen.getByText('トークンが無効です。')).toBeInTheDocument();

            // Act: retry with correct token (200)
            vi.mocked(fetch).mockImplementation(() => mockFetchResponse(200));
            await user.click(screen.getByRole('button', { name: '接続' }));

            // Assert: error message is cleared
            expect(screen.queryByText('トークンが無効です。')).not.toBeInTheDocument();
        });

        it('should disable submit button when input contains only whitespace', async () => {
            // Arrange
            const user = userEvent.setup();
            render(<TokenGate onSubmit={vi.fn()} />);

            // Act
            await user.type(screen.getByPlaceholderText('Bearer token'), '   ');

            // Assert: trimmed value is empty, button stays disabled
            expect(screen.getByRole('button', { name: '接続' })).toBeDisabled();
        });
    });
});
