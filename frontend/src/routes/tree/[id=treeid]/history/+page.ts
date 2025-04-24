import type { Load } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';

type Response = {
	id: string;
};

export const load: Load = async ({ params }): Promise<Response> => {
	const treeId = params.id;

	if (!treeId) {
		error(400, {
			message: 'Tree id not specified.'
		});
	}

	return {
		id: treeId
	};
};
