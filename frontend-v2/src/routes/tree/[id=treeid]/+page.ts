import type { Load } from '@sveltejs/kit';
import { Tree } from '$lib/samples';

export const prerender = true;

export const load: Load = async (xyz) => {
	const { params } = xyz;

	return {
		id: params.id,
		tree: Tree,
	};
};
