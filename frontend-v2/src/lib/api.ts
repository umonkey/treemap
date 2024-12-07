import type { IStats, ITree } from '$lib/types';

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

	private async request<T>(method: string, path: string): Promise<Response<T>> {
		console.debug(`[api] Requesting ${method} ${this.root}${path}`);

		const request = new Request(this.root + path, {
			method
		});

		const response = await fetch(request);

		return {
			status: response.status,
			data: await response.json()
		};
	}
}

export const apiClient = new ApiClient();
