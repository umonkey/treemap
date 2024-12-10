import type { ILoginResponse, IMarkers, IStats, ITree, ITreeUpdatePayload } from '$lib/types';
import { isAuthenticated, authState } from '$lib/stores/auth';
import { get } from 'svelte/store';

interface Response<T> {
	status: number;
	data: T;
}

export class ApiClient {
	private root: string;

	constructor() {
		this.root = import.meta.env.VITE_API_ROOT;
		console.debug(`[api] Root: ${this.root}`);
	}

	public async getTree(id: string): Promise<Response<ITree>> {
		console.debug(`[api] Getting tree ${id}`);
		return await this.request('GET', `v1/trees/${id}`);
	}

	public async getStats(): Promise<Response<IStats>> {
		console.debug(`[api] Getting stats`);
		return await this.request('GET', 'v1/trees/stats');
	}

	public async getMarkers(
		n: number,
		e: number,
		s: number,
		w: number
	): Promise<Response<IMarkers[]>> {
		const search = new URLSearchParams({
			n: n.toString(),
			e: e.toString(),
			s: s.toString(),
			w: w.toString()
		});

		return await this.request('GET', 'v1/trees?' + search.toString());
	}

	public async loginWithGoogle(token: string): Promise<Response<ILoginResponse>> {
		console.debug('[api] Logging in with Google');

		return await this.request('POST', 'v2/login/google', {
			token,
			body: JSON.stringify({ token }),
			headers: {
				'Content-Type': 'application/json'
			}
		});
	}

	public async updateTree(id: string, props: ITreeUpdatePayload): Promise<Response<ITree>> {
		return await this.request('PUT', `v1/trees/${id}`, {
			body: JSON.stringify(props),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async addComment(id: string, message: string): Promise<Response<void>> {
		return await this.request('POST', `v1/trees/${id}/comments`, {
			body: JSON.stringify({ message }),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	/**
	 * Send a raw request to the API.
	 *
	 * @docs https://developer.mozilla.org/en-US/docs/Web/API/RequestInit
	 */
	private async request<T>(
		method: string,
		path: string,
		options?: RequestInit
	): Promise<Response<T>> {
		console.debug(`[api] Requesting ${method} ${this.root}${path}`);

		const request = new Request(this.root + path, {
			method,
			...options
		});

		const response = await fetch(request);
		const data = response.status == 202 ? undefined : await response.json();

		return {
			status: response.status,
			data
		};
	}

	private getAuthHeaders() {
		if (get(isAuthenticated)) {
			const token = get(authState).token;

			return {
				Authorization: `Bearer ${token}`
			};
		}

		return {};
	}
}

export const apiClient = new ApiClient();
