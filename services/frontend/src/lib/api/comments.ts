import type { ICommentList, IResponse } from '$lib/types';
import { getAuthHeaders, request } from './client';

export async function addComment(id: string, message: string): Promise<IResponse<void>> {
	const headers: HeadersInit = {
		'Content-Type': 'application/json',
		...getAuthHeaders()
	};

	return await request('POST', `v1/trees/${id}/comments`, {
		body: JSON.stringify({ message }),
		headers
	});
}

export async function getTreeComments(id: string): Promise<IResponse<ICommentList>> {
	return await request('GET', `v1/trees/${id}/comments`);
}

export async function getRecentComments(): Promise<IResponse<ICommentList>> {
	return await request('GET', 'v1/comments');
}
