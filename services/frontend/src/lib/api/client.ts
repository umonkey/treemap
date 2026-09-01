import { config } from '$lib/env';
import { authStore, isAuthenticated } from '$lib/stores/authStore';
import type { IRawError, IResponse } from '$lib/types';
import { Response } from '$lib/types_response';
import { get } from 'svelte/store';

export async function request<T>(
	method: string,
	path: string,
	options?: RequestInit
): Promise<IResponse<T>> {
	const root = config.apiRoot;
	const start = performance.now();

	try {
		const request = new Request(root + path, {
			method,
			...options
		});

		const response = await fetch(request);

		if (response.status === 204 || response.status === 202) {
			return {
				status: response.status,
				data: undefined,
				error: undefined
			};
		}

		if (response.status >= 400) {
			let errorObj = {
				code: 'http_error',
				description: response.statusText || `HTTP error ${response.status}`
			};

			try {
				const err: IRawError = await response.json();
				if (err?.error) {
					errorObj = err.error;
				}
			} catch {
				// Non-JSON response body (e.g., HTML error page from gateway)
			}

			return {
				status: response.status,
				data: undefined,
				error: errorObj
			};
		}

		return {
			status: response.status,
			data: await response.json(),
			error: undefined
		};
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
	} catch (e: any) {
		const duration = Math.round(performance.now() - start);

		console.warn(`[api] Sent ${method} to /${path} in ${duration} ms`, e);

		return {
			status: 500,
			data: undefined,
			error: {
				code: 'network_error',
				description: (e as unknown as Error).message
			}
		};
	}
}

export function getAuthHeaders(): HeadersInit {
	if (get(isAuthenticated)) {
		const token = get(authStore)?.token;

		if (token) {
			return {
				Authorization: `Bearer ${token}`
			};
		}
	}

	return {};
}

export const unwrap = <T>(res: IResponse<T>): Response<T> => {
	return new Response<T>(res.status, res.data, res.error);
};
