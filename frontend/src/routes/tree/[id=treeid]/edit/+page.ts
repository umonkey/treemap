import type { Load } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';

export const load: Load = ({ params }) => {
	const id = params.id;

	if (!id) {
		error(404);
	}

	return {
		id
	};
};
