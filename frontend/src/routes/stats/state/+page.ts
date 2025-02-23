import type { Load } from '@sveltejs/kit';
import { loader } from './loader';

export const load: Load = async () => {
	return await loader();
};
