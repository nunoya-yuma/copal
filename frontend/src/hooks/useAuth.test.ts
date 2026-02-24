import { describe, it, expect, beforeEach } from 'vitest';
import { useAuth } from './useAuth';
import { renderHook, act } from '@testing-library/react';

const STORAGE_KEY = 'copal_api_token';

describe('useAuth', () => {
    beforeEach(() => {
        localStorage.clear();
    });

    describe('Initial state', () => {
        it('should start unauthenticated when localStorage is empty', () => {
            const sut = renderHook(() => useAuth());

            expect(sut.result.current.token).toBe('');
            expect(sut.result.current.isAuthenticated).toBeFalsy();
        });

        it('should restore token from localStorage on mount', () => {
            localStorage.setItem(STORAGE_KEY, 'persisted-token');

            const sut = renderHook(() => useAuth());

            expect(sut.result.current.token).toBe('persisted-token');
            expect(sut.result.current.isAuthenticated).toBeTruthy();
        });
    });

    describe('setToken', () => {
        it('should update token state', async () => {
            const sut = renderHook(() => useAuth());

            await act(async () => {
                sut.result.current.setToken('new-token');
            });

            expect(sut.result.current.token).toBe('new-token');
            expect(sut.result.current.isAuthenticated).toBeTruthy();
        });

        it('should persist token to localStorage', async () => {
            const sut = renderHook(() => useAuth());

            await act(async () => {
                sut.result.current.setToken('new-token');
            });

            expect(localStorage.getItem(STORAGE_KEY)).toBe('new-token');
        });
    });

    describe('clearToken', () => {
        it('should reset token state', async () => {
            const sut = renderHook(() => useAuth());
            await act(async () => { sut.result.current.setToken('existing-token'); });

            await act(async () => {
                sut.result.current.clearToken();
            });

            expect(sut.result.current.token).toBe('');
            expect(sut.result.current.isAuthenticated).toBeFalsy();
        });

        it('should remove token from localStorage', async () => {
            const sut = renderHook(() => useAuth());
            await act(async () => { sut.result.current.setToken('existing-token'); });

            await act(async () => {
                sut.result.current.clearToken();
            });

            expect(localStorage.getItem(STORAGE_KEY)).toBeNull();
        });
    });
});
