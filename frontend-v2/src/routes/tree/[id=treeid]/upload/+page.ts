import type { Load } from '@sveltejs/kit';

export const load: Load = ({
	params
}): Promise<{
	id: string;
}> => {
	return {
		id: params.id
	};
};
