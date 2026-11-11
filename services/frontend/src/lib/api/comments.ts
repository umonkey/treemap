import type { CommentEntityType, ICommentList, IResponse } from '$lib/types';
import { getAuthHeaders, request } from './client';

export async function addComment(
	entityType: CommentEntityType,
	entityId: string,
	message: string
): Promise<IResponse<void>> {
	const headers: HeadersInit = {
		'Content-Type': 'application/json',
		...getAuthHeaders()
	};

	return await request('POST', `v1/comments/${entityType}/${entityId}`, {
		body: JSON.stringify({ message }),
		headers
	});
}

export async function getComments(
	entityType: CommentEntityType,
	entityId: string
): Promise<IResponse<ICommentList>> {
	return await request('GET', `v1/comments/${entityType}/${entityId}`);
}

export async function getRecentComments(): Promise<IResponse<ICommentList>> {
	return await request('GET', 'v1/comments');
}
