import { listSpecies } from '$lib/api/species';
import type { ISpeciesListItem, IError } from '$lib/types';

export class PageState {
	species = $state<ISpeciesListItem[]>([]);
	loading = $state<boolean>(true);
	error = $state<IError | null>(null);

	reload = async () => {
		this.loading = true;
		this.error = null;

		try {
			const res = await listSpecies();
			if (res.status === 200 && res.data) {
				this.species = res.data;
			} else {
				this.error = res.error || {
					code: 'load_failed',
					description: `Error ${res.status} loading species.`
				};
			}
		} catch (err) {
			console.error('Failed to reload species:', err);
			this.error = { code: 'exception', description: 'Failed to reload species.' };
		} finally {
			this.loading = false;
		}
	};
}
