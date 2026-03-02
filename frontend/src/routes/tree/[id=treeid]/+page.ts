import { apiClient } from '$lib/api';
import { addUsers } from '$lib/stores/userStore';
import type { IComment, IObservation, ITree } from '$lib/types';
import type { Load } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';

export const load: Load = async ({
	params
}): Promise<{
	id: string;
	tree: ITree;
	comments: IComment[];
	observation: IObservation | null;
}> => {
	const treeId = params.id;

	if (!treeId) {
		error(404);
	}

	const [treeRes, commentsRes, observationRes] = await Promise.all([
		apiClient.getTree(treeId),
		apiClient.getTreeComments(treeId),
		apiClient.getObservations(treeId)
	]);

	if (treeRes.status === 200 && treeRes.data) {
		addUsers(treeRes.data.users);

		let comments: IComment[] = [];
		if (commentsRes.status === 200 && commentsRes.data) {
			addUsers(commentsRes.data.users);
			comments = commentsRes.data.comments;
		}

		return {
			id: treeId,
			tree: treeRes.data,
			comments,
			observation: observationRes.data ?? null
		};
	}

	error(treeRes.status);
};
