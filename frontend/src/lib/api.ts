import { API_ROOT } from '$lib/env';
import { authStore, isAuthenticated } from '$lib/stores/authStore';
import { addUsers } from '$lib/stores/userStore';
import { addTrees, getTree } from '$lib/stores/treeStore';
import type {
	IAddTreesRequest,
	IChangeList,
	ICommentList,
	ILikeList,
	ILoginResponse,
	IMarkers,
	IMeResponse,
	IRawError,
	IReplaceTreeRequest,
	IResponse,
	ISingleTree,
	ISpecies,
	IStreet,
	ISpeciesStats,
	IStateStats,
	IStats,
	IStreetStats,
	ITree,
	ITreeDefaults,
	ITreeFile,
	ITreeList,
	ITreeUpdatePayload,
	IUploadResponse,
	DuplicateList
} from '$lib/types';
import { Response } from '$lib/types_response';
import { get } from 'svelte/store';

export class ApiClient {
	private root: string;

	constructor() {
		this.root = API_ROOT;
		// console.debug(`[api] Root: ${this.root}`);
	}

	// Return a single tree.
	public async getTree(id: string, nocache?: boolean): Promise<IResponse<ISingleTree>> {
		const cached = get(getTree)(id);

		if (cached && !nocache) {
			console.debug(`[api] Tree ${id} found in cache.`);

			return {
				status: 200,
				data: {
					...cached,
					users: []
				},
				error: undefined
			};
		}

		const res = await this.request<ISingleTree>('GET', `v1/trees/${id}`);

		if (res.status === 200 && res.data) {
			addTrees([res.data]);
			addUsers(res.data.users);
		}

		return res;
	}

	public async getTreeFiles(id: string): Promise<IResponse<ITreeFile[]>> {
		const res = await this.getTree(id, true);

		if (res.status === 200 && res.data) {
			return {
				status: 200,
				data: res.data.files ?? [],
				error: undefined
			};
		}

		return {
			status: res.status,
			data: undefined,
			error: res.error
		};
	}

	public async getTreeDefaults(): Promise<IResponse<ITreeDefaults>> {
		return await this.request<ITreeDefaults>('GET', 'v1/trees/defaults');
	}

	public async getStats(): Promise<IResponse<IStats>> {
		return await this.request('GET', 'v1/trees/stats');
	}

	public async getSpeciesStats(): Promise<IResponse<ISpeciesStats[]>> {
		return await this.request('GET', 'v1/stats/species');
	}

	public async getSpeciesMismatch(): Promise<IResponse<ITreeList>> {
		return await this.request('GET', 'v1/stats/species/mismatch');
	}

	public async getTopHeight(): Promise<IResponse<ITreeList>> {
		return await this.request('GET', 'v1/stats/height');
	}

	public async getTopDiameter(): Promise<IResponse<ITreeList>> {
		return await this.request('GET', 'v1/stats/diameter');
	}

	public async getTopCircumference(): Promise<IResponse<ITreeList>> {
		return await this.request('GET', 'v1/stats/circumference');
	}

	public async getTopStreets(): Promise<IResponse<IStreetStats[]>> {
		return await this.request('GET', 'v1/stats/streets');
	}

	public async getStateStats(): Promise<IResponse<IStateStats[]>> {
		return await this.request('GET', 'v1/stats/state');
	}

	public async getMe(): Promise<IResponse<IMeResponse>> {
		return await this.request('GET', 'v1/me', {
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	// Update user's display name and profile picture.
	public async updateSettings({
		name,
		picture
	}: {
		name: string;
		picture: string | null;
	}): Promise<IResponse<void>> {
		return await this.request('PUT', `v1/settings`, {
			body: JSON.stringify({ name, picture }),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async getMeLikes(): Promise<IResponse<ILikeList>> {
		return await this.request('GET', 'v1/me/likes', {
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async verifyToken(token: string): Promise<IResponse<IMeResponse>> {
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
		search?: string | null
	): Promise<IResponse<IMarkers>> {
		const params = new URLSearchParams({
			n: n.toString(),
			e: e.toString(),
			s: s.toString(),
			w: w.toString()
		});

		if (search) {
			params.set('search', search);
		}

		const res = await this.request<IMarkers>('GET', `v1/trees?${params.toString()}`);

		if (res.status === 200 && res.data) {
			addTrees(res.data.trees);
		}

		return res;
	}

	public async addTree(props: IAddTreesRequest): Promise<IResponse<ITreeList>> {
		return await this.request('POST', 'v1/trees', {
			body: JSON.stringify(props),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async replaceTree(id: string, props: IReplaceTreeRequest): Promise<IResponse<ITreeList>> {
		return await this.request('PUT', `v1/trees/${id}/replace`, {
			body: JSON.stringify(props),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async addTraining(result: number): Promise<IResponse<void>> {
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

	public async loginWithGoogle(token: string): Promise<IResponse<ILoginResponse>> {
		console.debug('[api] Logging in with Google');

		return await this.request('POST', 'v2/login/google', {
			body: JSON.stringify({ token }),
			headers: {
				'Content-Type': 'application/json'
			}
		});
	}

	public async updateTree(id: string, props: ITreeUpdatePayload): Promise<IResponse<ITree>> {
		const res = await this.request<ITree>('PUT', `v1/trees/${id}`, {
			body: JSON.stringify(props),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});

		if (res.status === 200 && res.data) {
			console.debug(`[api] Tree ${id} updated in cache.`);
			addTrees([res.data]);
		}

		return res;
	}

	public async updateTreeHeight(id: string, value: number): Promise<IResponse<ITree>> {
		const res = await this.request<ITree>('PUT', `v1/trees/${id}/height`, {
			body: JSON.stringify({ value }),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});

		if (res.status === 200 && res.data) {
			addTrees([res.data]);
		}

		return res;
	}

	public async updateTreeLocation(id: string, lat: number, lon: number): Promise<IResponse<ITree>> {
		const res = await this.request<ITree>('PUT', `v1/trees/${id}/location`, {
			body: JSON.stringify({ lat, lon }),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});

		if (res.status === 200 && res.data) {
			addTrees([res.data]);
		}

		return res;
	}

	public async updateTreeDiameter(id: string, value: number): Promise<IResponse<ITree>> {
		const res = await this.request<ITree>('PUT', `v1/trees/${id}/diameter`, {
			body: JSON.stringify({ value }),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});

		if (res.status === 200 && res.data) {
			addTrees([res.data]);
		}

		return res;
	}

	public async updateTreeCircumference(id: string, value: number): Promise<IResponse<ITree>> {
		const res = await this.request<ITree>('PUT', `v1/trees/${id}/circumference`, {
			body: JSON.stringify({ value }),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});

		if (res.status === 200 && res.data) {
			addTrees([res.data]);
		}

		return res;
	}

	public async updateTreeState(
		id: string,
		value: string | null,
		comment?: string
	): Promise<IResponse<ITree>> {
		const payload: { value: string | null; comment?: string } = { value };
		if (comment && comment.trim()) {
			payload.comment = comment.trim();
		}

		const res = await this.request<ITree>('PUT', `v1/trees/${id}/state`, {
			body: JSON.stringify(payload),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});

		if (res.status === 200 && res.data) {
			addTrees([res.data]);
		}

		return res;
	}

	public async addComment(id: string, message: string): Promise<IResponse<void>> {
		const headers: HeadersInit = {
			'Content-Type': 'application/json',
			...this.getAuthHeaders()
		};

		return await this.request('POST', `v1/trees/${id}/comments`, {
			body: JSON.stringify({ message }),
			headers
		});
	}

	/**
	 * This is deprecated.
	 * Please use uploadSingleFile for now.
	 **/
	public async uploadFile(tree: string, file: File): Promise<IResponse<void>> {
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

	// Add previously uploaded files as photos to a tree.
	public async addPhotos(tree_id: string, files: string[]): Promise<IResponse<void>> {
		const headers: HeadersInit = {
			'Content-Type': 'application/json',
			...this.getAuthHeaders()
		};

		return await this.request('POST', `v1/trees/${tree_id}/photos`, {
			body: JSON.stringify({ files }),
			headers
		});
	}

	/**
	 * This is how you upload all files.
	 * The response is the file id, which is then used for adding photos to trees.
	 */
	public async uploadSingleFile(file: File): Promise<IResponse<IUploadResponse>> {
		const headers: HeadersInit = {
			'Content-Type': 'application/json',
			...this.getAuthHeaders()
		};

		const buffer = await file.arrayBuffer();
		const body = new Blob([buffer], { type: file.type });

		return await this.request('POST', `v1/upload`, {
			body,
			headers
		});
	}

	public async searchSpecies(query: string): Promise<IResponse<ISpecies[]>> {
		const params = new URLSearchParams({ query });
		return await this.request('GET', `v1/species/search?${params}`);
	}

	public async searchStreets(query: string): Promise<IResponse<IStreet[]>> {
		const params = new URLSearchParams({ query });
		return await this.request('GET', `v1/streets/search?${params}`);
	}

	public async suggestSpecies(): Promise<IResponse<string[]>> {
		return await this.request('GET', 'v1/species/suggest', {
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async getTreeComments(id: string): Promise<IResponse<ICommentList>> {
		return await this.request('GET', `v1/trees/${id}/comments`);
	}

	public async getTreeHistory(id: string): Promise<IResponse<IChangeList>> {
		return await this.request('GET', `v1/trees/${id}/history`);
	}

	public async getRecentComments(): Promise<IResponse<ICommentList>> {
		return await this.request('GET', 'v1/comments');
	}

	public async getNewTrees(): Promise<IResponse<ITreeList>> {
		return await this.request('GET', 'v1/trees/new');
	}

	public async getUpdatedTrees(): Promise<IResponse<ITreeList>> {
		return await this.request('GET', 'v1/trees/updated');
	}

	public async changeTreeThumbnail(tree: string, file: string): Promise<IResponse<void>> {
		return await this.request('PUT', `v1/trees/${tree}/thumbnail`, {
			body: JSON.stringify({ file }),
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async deleteFile(id: string): Promise<IResponse<void>> {
		return await this.request('DELETE', `v1/files/${id}`, {
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async likeTree(id: string): Promise<IResponse<void>> {
		return await this.request('POST', `v1/trees/${id}/likes`, {
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async unlikeTree(id: string): Promise<IResponse<void>> {
		return await this.request('DELETE', `v1/trees/${id}/likes`, {
			headers: {
				'Content-Type': 'application/json',
				...this.getAuthHeaders()
			}
		});
	}

	public async getDuplicates(): Promise<IResponse<DuplicateList>> {
		const res = await this.request<DuplicateList>('GET', 'v1/duplicates');
		return res;
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
	): Promise<IResponse<T>> {
		console.debug(`[api] Requesting ${method} ${this.root}${path}`);

		try {
			const request = new Request(this.root + path, {
				method,
				...options
			});

			const response = await fetch(request);

			if (response.status === 202) {
				return {
					status: 202,
					data: undefined,
					error: undefined
				};
			}

			if (response.status >= 400) {
				const err: IRawError = await response.json();

				return {
					status: response.status,
					data: undefined,
					error: err.error
				};
			}

			return {
				status: response.status,
				data: await response.json(),
				error: undefined
			};

			// eslint-disable-next-line @typescript-eslint/no-explicit-any
		} catch (e: any) {
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

export const unwrap = <T>(res: IResponse<T>): Response<T> => {
	return new Response<T>(res.status, res.data, res.error);
};

export const apiClient = new ApiClient();
