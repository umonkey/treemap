import type { Load } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';

export const load: Load = ({
	params
}): {
	id: string;
} => {
	const treeId = params.id;

	if (!treeId) {
		error(404);
	}

	return {
		id: treeId
	};
};
