import { apiClient } from '$lib/api';
import type { IStateStats } from '$lib/types';

const DEFAULT_ERROR = 'An error occurred while loading stats. Please try again later.';

export const loader = async (): Promise<{
	error: string | null;
	stats: IStateStats[];
}> => {
	let error: string | null = null;
	let stats: IStateStats[] = [];

	try {
		console.debug('Loading state stats...');

		const res = await apiClient.getStateStats();

		if (res.status === 200 && res.data) {
			stats = res.data;
		} else if (res.error) {
			error = res.error.error.description ?? DEFAULT_ERROR;
		}
	} catch (e) {
		console.error('Error loading stats:', e);
		error = (e as Error).message;
	}

	return { stats, error };
};
