import { addSpecies } from '$lib/api/species';
import { showError } from '$lib/errors';
import { goto } from '$app/navigation';

// Split a comma- or semicolon-separated keyword string into a list.
const parseKeywords = (raw: string): string[] =>
	raw
		.split(/[;,]/)
		.map((keyword) => keyword.trim())
		.filter((keyword) => keyword.length > 0);

export class PageState {
	name = $state<string>('');
	local = $state<string>('');
	keywords = $state<string>('');
	wikidataId = $state<string>('');
	saving = $state<boolean>(false);

	handleSave = async () => {
		if (this.saving) return;
		this.saving = true;

		try {
			const res = await addSpecies({
				name: this.name,
				local: this.local,
				keywords: parseKeywords(this.keywords),
				wikidata_id: this.wikidataId
			});

			if (res.status >= 200 && res.status < 300 && res.data) {
				goto(`/tools/species/${encodeURIComponent(res.data.name)}`);
			} else {
				showError(res.error?.description || `Error ${res.status} creating species.`);
			}
		} catch (err) {
			console.error('Failed to create species:', err);
			showError('Failed to create species.');
		} finally {
			this.saving = false;
		}
	};
}
