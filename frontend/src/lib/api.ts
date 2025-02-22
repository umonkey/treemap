import type {
	IAddTreesRequest,
	ICommentList,
	ILikeList,
	ILoginResponse,
	IMarkers,
	IMeResponse,
	ISingleTree,
	ISpecies,
	ISpeciesStats,
	IStats,
	IStreetStats,
	ITree,
	ITreeDefaults,
	ITreeList,
	ITreeUpdatePayload,
} from '$lib/types';
import { isAuthenticated, authStore } from '$lib/stores/authStore';
import { addUsers } from '$lib/stores/userStore';
import { get } from 'svelte/store';
import { API_ROOT } from '$lib/env';

interface Response<T> {
	status: number;
	data: T;
}

export class ApiClient {
	private root: string;

	constructor() {
		this.root = API_ROOT;
		console.debug(`[api] Root: ${this.root}`);
	}

	public async getTree(id: string): Promise<Response<ISingleTree>> {
		console.debug(`[api] Getting tree ${id}`);
		const res = await this.request<ISingleTree>('GET', `v1/trees/${id}`);

		if (res.status === 200) {
			addUsers(res.data.users);
		}

		return res;
	}

	public async getTreeDefaults(): Promise<Response<ITreeDefaults>> {
		console.debug(`[api] Getting tree defaults`);
		return await this.request<ITreeDefaults>('GET', 'v1/trees/defaults');
	}

	public async getStats(): Promise<Response<IStats>> {
		return await this.request('GET', 'v1/trees/stats');
	}

	public async getSpeciesStats(): Promise<Response<ISpeciesStats[]>> {
		return await this.request('GET', 'v1/stats/species');
	}

	public async getSpeciesMismatch(): Promise<Response<ITreeList>> {
		return await this.request('GET', 'v1/stats/species/mismatch');
	}

	public async getTopHeight(): Promise<Response<ITreeList>> {
		return await this.request('GET', 'v1/stats/height');
	}

	public async getTopDiameter(): Promise<Response<ITreeList>> {
		return await this.request('GET', 'v1/stats/diameter');
	}

	public async getTopCircumference(): Promise<Response<ITreeList>> {
		return await this.request('GET', 'v1/stats/circumference');
	}

	public async getTopStreets(): Promise<Response<IStreetStats[]>> {
		return await this.request('GET', 'v1/stats/streets');
	}

	public async getMe(): Promise<Response<IMeResponse>> {
		return await this.request('GET', 'v1/me', {
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async getMeLikes(): Promise<Response<ILikeList>> {
		return await this.request('GET', 'v1/me/likes', {
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async verifyToken(token: string): Promise<Response<IMeResponse>> {
		return await this.request('GET', 'v1/me', {
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${token}`
			}
		});
	}

	public async getMarkers(
		n: number,
		e: number,
		s: number,
		w: number,
		search?: string | undefined
	): Promise<Response<IMarkers>> {
		const params = new URLSearchParams({
			n: n.toString(),
			e: e.toString(),
			s: s.toString(),
			w: w.toString()
		});

		if (search) {
			params.set('search', search);
		}

		return await this.request('GET', 'v1/trees?' + params.toString());
	}

	public async addTree(props: IAddTreesRequest): Promise<Response<ITreeList>> {
		return await this.request('POST', 'v1/trees', {
			body: JSON.stringify(props),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async addTraining(result: number): Promise<Response<void>> {
		return await this.request('POST', 'v1/training', {
			body: JSON.stringify({
				result
			}),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async loginWithGoogle(token: string): Promise<Response<ILoginResponse>> {
		console.debug('[api] Logging in with Google');

		return await this.request('POST', 'v2/login/google', {
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

	public async updateTreeHeight(id: string, value: number): Promise<Response<ITree>> {
		return await this.request('PUT', `v1/trees/${id}/height`, {
			body: JSON.stringify({ value }),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async updateTreeDiameter(id: string, value: number): Promise<Response<ITree>> {
		return await this.request('PUT', `v1/trees/${id}/diameter`, {
			body: JSON.stringify({ value }),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async updateTreeCircumference(id: string, value: number): Promise<Response<ITree>> {
		return await this.request('PUT', `v1/trees/${id}/circumference`, {
			body: JSON.stringify({ value }),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async updateTreeState(id: string, value: string | null): Promise<Response<ITree>> {
		return await this.request('PUT', `v1/trees/${id}/state`, {
			body: JSON.stringify({ value }),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async addComment(id: string, message: string): Promise<Response<void>> {
		const headers: HeadersInit = {
			'Content-Type': 'application/json',
			...this.getAuthHeaders()
		};

		return await this.request('POST', `v1/trees/${id}/comments`, {
			body: JSON.stringify({ message }),
			headers
		});
	}

	public async uploadFile(tree: string, file: File): Promise<Response<void>> {
		const headers: HeadersInit = {
			'Content-Type': 'application/json',
			...this.getAuthHeaders()
		};

		const buffer = await file.arrayBuffer();
		const body = new Blob([buffer], { type: file.type });

		return await this.request('POST', `v1/trees/${tree}/files`, {
			body,
			headers
		});
	}

	public async searchSpecies(query: string): Promise<Response<ISpecies[]>> {
		const params = new URLSearchParams({ query });
		return await this.request('GET', `v1/species/search?${params}`);
	}

	public async getTreeComments(id: string): Promise<Response<ICommentList>> {
		return await this.request('GET', `v1/trees/${id}/comments`);
	}

	public async getRecentComments(): Promise<Response<ICommentList>> {
		return await this.request('GET', `v1/comments`);
	}

	public async getNewTrees(): Promise<Response<ITreeList>> {
		return await this.request('GET', `v1/trees/new`);
	}

	public async getUpdatedTrees(): Promise<Response<ITreeList>> {
		return await this.request('GET', `v1/trees/updated`);
	}

	public async changeTreeThumbnail(tree: string, file: string): Promise<Response<void>> {
		return await this.request('PUT', `v1/trees/${tree}/thumbnail`, {
			body: JSON.stringify({ file }),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async deleteFile(id: string): Promise<Response<void>> {
		return await this.request('DELETE', `v1/files/${id}`, {
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async likeTree(id: string): Promise<Response<void>> {
		return await this.request('POST', `v1/trees/${id}/likes`, {
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async unlikeTree(id: string): Promise<Response<void>> {
		return await this.request('DELETE', `v1/trees/${id}/likes`, {
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

	private getAuthHeaders(): HeadersInit {
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
}

export const apiClient = new ApiClient();
