import type { Load } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';

export const load: Load = ({
	params
}): Promise<{
	id: string;
}> => {
	const id = params.id;

	if (!id) {
		error(404);
	}

	return {
		id
	};
};
