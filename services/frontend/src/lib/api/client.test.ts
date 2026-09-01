import { describe, expect, it, vi, beforeEach, afterEach } from 'vitest';
import { request } from './client';

describe('API Client request', () => {
	const originalFetch = global.fetch;
	const originalWarn = console.warn;

	beforeEach(() => {
		vi.stubGlobal('fetch', vi.fn());
		console.warn = vi.fn();
	});

	afterEach(() => {
		global.fetch = originalFetch;
		console.warn = originalWarn;
		vi.restoreAllMocks();
	});

	it('should handle successful JSON responses', async () => {
		const mockResponse = {
			ok: true,
			status: 200,
			json: async () => ({ message: 'success' })
		};
		vi.mocked(fetch).mockResolvedValueOnce(mockResponse as unknown as Response);

		const res = await request<{ message: string }>('GET', 'v1/test');

		expect(res.status).toBe(200);
		expect(res.data).toEqual({ message: 'success' });
		expect(res.error).toBeUndefined();
	});

	it('should handle 204 No Content responses', async () => {
		const mockResponse = {
			ok: true,
			status: 204
		};
		vi.mocked(fetch).mockResolvedValueOnce(mockResponse as unknown as Response);

		const res = await request<void>('DELETE', 'v1/test/1');

		expect(res.status).toBe(204);
		expect(res.data).toBeUndefined();
		expect(res.error).toBeUndefined();
	});

	it('should handle 402/400 JSON error responses', async () => {
		const mockResponse = {
			ok: false,
			status: 400,
			json: async () => ({
				error: {
					code: 'bad_request',
					description: 'Invalid input'
				}
			})
		};
		vi.mocked(fetch).mockResolvedValueOnce(mockResponse as unknown as Response);

		const res = await request<void>('POST', 'v1/test');

		expect(res.status).toBe(400);
		expect(res.data).toBeUndefined();
		expect(res.error).toEqual({
			code: 'bad_request',
			description: 'Invalid input'
		});
	});

	it('should handle non-JSON error responses (e.g. HTML 502 Bad Gateway)', async () => {
		const mockResponse = {
			ok: false,
			status: 502,
			statusText: 'Bad Gateway',
			json: async () => {
				throw new SyntaxError('Unexpected token < in JSON at position 0');
			}
		};
		vi.mocked(fetch).mockResolvedValueOnce(mockResponse as unknown as Response);

		const res = await request<void>('GET', 'v1/test');

		expect(res.status).toBe(502);
		expect(res.data).toBeUndefined();
		expect(res.error).toEqual({
			code: 'http_error',
			description: 'Bad Gateway'
		});
	});

	it('should handle network fetch errors and log via console.warn', async () => {
		vi.mocked(fetch).mockRejectedValueOnce(new TypeError('Load failed'));

		const res = await request<void>('GET', 'v1/test');

		expect(res.status).toBe(500);
		expect(res.data).toBeUndefined();
		expect(res.error).toEqual({
			code: 'network_error',
			description: 'Load failed'
		});
		expect(console.warn).toHaveBeenCalledTimes(1);
	});
});
