import type { IHeatMap, ILikeList, IMeResponse, IResponse, IUser, IUserList } from '$lib/types';
import { getAuthHeaders, request } from './client';

export async function getMe(): Promise<IResponse<IMeResponse>> {
	return await request('GET', 'v1/me', {
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
}

// Update user's display name and profile picture.
export async function updateSettings({
	name,
	picture
}: {
	name: string;
	picture: string | null;
}): Promise<IResponse<void>> {
	return await request('PUT', 'v1/settings', {
		body: JSON.stringify({ name, picture }),
		headers: {
			'Content-Type': 'application/json',
			...getAuthHeaders()
		}
	});
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

export async function verifyToken(token: string): Promise<IResponse<IMeResponse>> {
	return await request('GET', 'v1/me', {
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${token}`
		}
	});
}

export async function getUserHeatMap(id: string): Promise<IResponse<IHeatMap[]>> {
	return await request('GET', `v1/users/${id}/heatmap`);
}
