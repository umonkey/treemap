import { getSpecies, updateSpecies } from '$lib/api/species';
import { showError } from '$lib/errors';
import { goto } from '$app/navigation';
import type { IError } from '$lib/types';

// Split a comma- or semicolon-separated keyword string into a list.
export const parseKeywords = (raw: string): string[] =>
	raw
		.split(/[;,]/)
		.map((keyword) => keyword.trim())
		.filter((keyword) => keyword.length > 0);

export class PageState {
	name = $state<string>('');
	local = $state<string>('');
	keywords = $state<string>('');
	wikidataId = $state<string>('');
	loading = $state<boolean>(true);
	saving = $state<boolean>(false);
	error = $state<IError | null>(null);
	currentName = $state<string>('');

	reload = async (name: string) => {
		if (this.currentName === name) return;
		this.currentName = name;
		this.loading = true;
		this.error = null;

		try {
			const res = await getSpecies(name);
			if (res.status === 200 && res.data) {
				this.name = res.data.name;
				this.local = res.data.local;
				this.keywords = res.data.keywords.join('; ');
				this.wikidataId = res.data.wikidata_id;
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

	handleSave = async () => {
		if (this.saving) return;
		this.saving = true;

		try {
			const res = await updateSpecies(this.currentName, {
				local: this.local,
				keywords: parseKeywords(this.keywords),
				wikidata_id: this.wikidataId
			});

			if (res.status >= 200 && res.status < 300) {
				goto(`/tools/species/${encodeURIComponent(this.currentName)}`);
			} else {
				showError(res.error?.description || `Error ${res.status} updating species.`);
			}
		} catch (err) {
			console.error('Failed to update species:', err);
			showError('Failed to update species.');
		} finally {
			this.saving = false;
		}
	};
}
