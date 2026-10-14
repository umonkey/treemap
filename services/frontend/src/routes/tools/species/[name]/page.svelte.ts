import { getSpecies } from '$lib/api/species';
import type { ISpeciesDetails, IError } from '$lib/types';

export class PageState {
	species = $state<ISpeciesDetails | null>(null);
	loading = $state<boolean>(true);
	error = $state<IError | null>(null);
	currentName = $state<string>('');

	reload = async (name: string) => {
		if (this.currentName === name && this.species) return;
		this.currentName = name;
		this.loading = true;
		this.error = null;

		try {
			const res = await getSpecies(name);
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
