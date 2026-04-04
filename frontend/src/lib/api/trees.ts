import { addTrees, getTree as getCachedTree } from '$lib/stores/treeStore';
import { addUsers } from '$lib/stores/userStore';
import type {
	DuplicateList,
	IAddTreesRequest,
	IChangeList,
	IMarkers,
	IReplaceTreeRequest,
	IResponse,
	ISingleTree,
	ITree,
	ITreeDefaults,
	ITreeFile,
	ITreeList,
	ITreeUpdatePayload,
	IUserList
} from '$lib/types';
import { get } from 'svelte/store';
import { getAuthHeaders, request } from './client';

// Return a single tree.
export async function getTree(id: string, nocache?: boolean): Promise<IResponse<ISingleTree>> {
	const cached = get(getCachedTree)(id);

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

	const res = await request<ISingleTree>('GET', `v1/trees/${id}`);

	if (res.status === 200 && res.data) {
		addTrees([res.data]);
		addUsers(res.data.users);
	}

	return res;
}

export async function getTreeFiles(id: string): Promise<IResponse<ITreeFile[]>> {
	const res = await getTree(id, true);

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

export async function getTreeDefaults(): Promise<IResponse<ITreeDefaults>> {
	return await request<ITreeDefaults>('GET', 'v1/trees/defaults');
}

export async function getMarkers(
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

	const res = await request<IMarkers>('GET', `v1/trees?${params.toString()}`);

	if (res.status === 200 && res.data) {
		addTrees(res.data.trees);
	}

	return res;
}

export async function getGeoJSON(
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

	return await request<IMarkers>('GET', `v1/trees/geo.json?${params.toString()}`);
}

export async function addTree(props: IAddTreesRequest): Promise<IResponse<ITreeList>> {
	return await request('POST', 'v1/trees', {
		body: JSON.stringify(props),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}

export async function replaceTree(
	id: string,
	props: IReplaceTreeRequest
): Promise<IResponse<ITreeList>> {
	return await request('PUT', `v1/trees/${id}/replace`, {
		body: JSON.stringify(props),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}

export async function updateTree(id: string, props: ITreeUpdatePayload): Promise<IResponse<ITree>> {
	const res = await request<ITree>('PUT', `v1/trees/${id}`, {
		body: JSON.stringify(props),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});

	if (res.status === 200 && res.data) {
		console.debug(`[api] Tree ${id} updated in cache.`);
		addTrees([res.data]);
	}

	return res;
}

export async function updateTreeHeight(id: string, value: number): Promise<IResponse<ITree>> {
	const res = await request<ITree>('PUT', `v1/trees/${id}/height`, {
		body: JSON.stringify({ value }),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});

	if (res.status === 200 && res.data) {
		addTrees([res.data]);
	}

	return res;
}

export async function updateTreeLocation(
	id: string,
	lat: number,
	lon: number
): Promise<IResponse<ITree>> {
	const res = await request<ITree>('PUT', `v1/trees/${id}/location`, {
		body: JSON.stringify({ lat, lon }),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});

	if (res.status === 200 && res.data) {
		addTrees([res.data]);
	}

	return res;
}

export async function updateTreeDiameter(id: string, value: number): Promise<IResponse<ITree>> {
	const res = await request<ITree>('PUT', `v1/trees/${id}/diameter`, {
		body: JSON.stringify({ value }),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});

	if (res.status === 200 && res.data) {
		addTrees([res.data]);
	}

	return res;
}

export async function updateTreeCircumference(
	id: string,
	value: number
): Promise<IResponse<ITree>> {
	const res = await request<ITree>('PUT', `v1/trees/${id}/circumference`, {
		body: JSON.stringify({ value }),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});

	if (res.status === 200 && res.data) {
		addTrees([res.data]);
	}

	return res;
}

export async function updateTreeState(
	id: string,
	value: string | null,
	comment?: string
): Promise<IResponse<ITree>> {
	const payload: { value: string | null; comment?: string } = { value };
	if (comment?.trim()) {
		payload.comment = comment.trim();
	}

	const res = await request<ITree>('PUT', `v1/trees/${id}/state`, {
		body: JSON.stringify(payload),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});

	if (res.status === 200 && res.data) {
		addTrees([res.data]);
	}

	return res;
}

export async function getTreeActors(id: string): Promise<IResponse<IUserList>> {
	return await request('GET', `v1/trees/${id}/actors`);
}

export async function getTreeHistory(id: string): Promise<IResponse<IChangeList>> {
	return await request('GET', `v1/trees/${id}/history`);
}

export async function getNewTrees(): Promise<IResponse<ITreeList>> {
	return await request('GET', 'v1/trees/new');
}

export async function getUpdatedTrees({
	count = 10,
	skip = 0
}: {
	count: number;
	skip: number;
}): Promise<IResponse<ITreeList>> {
	const params = new URLSearchParams({
		count: count.toString(),
		skip: skip.toString()
	});

	return await request('GET', `v1/trees/updated?${params.toString()}`);
}

export async function changeTreeThumbnail(tree: string, file: string): Promise<IResponse<void>> {
	return await request('PUT', `v1/trees/${tree}/thumbnail`, {
		body: JSON.stringify({ file }),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}

export async function likeTree(id: string): Promise<IResponse<void>> {
	return await request('POST', `v1/trees/${id}/likes`, {
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}

export async function unlikeTree(id: string): Promise<IResponse<void>> {
	return await request('DELETE', `v1/trees/${id}/likes`, {
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}

export async function getDuplicates(): Promise<IResponse<DuplicateList>> {
	const res = await request<DuplicateList>('GET', 'v1/duplicates');
	return res;
}
