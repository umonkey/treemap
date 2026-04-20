import type { IHeatMap, ILikeList, IMeResponse, IResponse, IUser, IUserList } from '$lib/types';
import { authStore } from '$lib/stores/authStore';
import { getAuthHeaders, request } from './client';

export async function getMe(): Promise<IResponse<IMeResponse>> {
	const res = await request<IMeResponse>('GET', 'v1/me', {
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});

	if (res.status === 200 && res.data) {
		const data = res.data;
		authStore.update((state) => {
			if (state) {
				return {
					...state,
					id: data.id,
					name: data.name,
					picture: data.picture
				};
			}
			return state;
		});
	}

	return res;
}

export async function verifyToken(token: string): Promise<IResponse<IMeResponse>> {
	const res = await request<IMeResponse>('GET', 'v1/me', {
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${token}`
		}
	});

	if (res.status === 200 && res.data) {
		const data = res.data;
		authStore.update((state) => {
			if (state && state.token === token) {
				return {
					...state,
					id: data.id,
					name: data.name,
					picture: data.picture
				};
			}
			return state;
		});
	}

	return res;
}

// Update user's display name and profile picture.
export async function updateSettings({
	name,
	picture
}: {
	name: string;
	picture: string | null;
}): Promise<IResponse<void>> {
	const res = await request<void>('PUT', 'v1/settings', {
		body: JSON.stringify({ name, picture }),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});

	if (res.status === 200) {
		authStore.update((state) => {
			if (state) {
				return {
					...state,
					name,
					picture: picture || state.picture
				};
			}
			return state;
		});
	}

	return res;
}

export async function getMeLikes(): Promise<IResponse<ILikeList>> {
	return await request('GET', 'v1/me/likes', {
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}

export async function getUsers(): Promise<IResponse<IUserList>> {
	return await request('GET', 'v1/users', {
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}

export async function getUser(id: string): Promise<IResponse<IUser>> {
	return await request('GET', `v1/users/${id}`, {
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}

export async function updateUser(id: string, props: Partial<IUser>): Promise<IResponse<IUser>> {
	return await request('PUT', `v1/users/${id}`, {
		body: JSON.stringify(props),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}

export async function getUserHeatMap(id: string): Promise<IResponse<IHeatMap[]>> {
	return await request('GET', `v1/users/${id}/heatmap`);
}
