import { getDuplicates } from '$lib/api/trees';
import type { DuplicateList } from '$lib/types';
import { writable } from 'svelte/store';

export const hooks = () => {
	const data = writable<DuplicateList | undefined>(undefined);
	const error = writable<string | null>(null);
	const loading = writable<boolean>(false);

	getDuplicates().then(({ status, data: d, error: e }) => {
		if (status === 200 && d) {
			data.set(d);
		} else if (e?.description) {
			error.set(e.description);
		}
	});

	return { data, loading };
};
