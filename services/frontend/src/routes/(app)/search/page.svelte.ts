import { goto, routes } from '$lib/routes';
import { mapLayerStore } from '$lib/stores/mapLayerStore';
import { get } from 'svelte/store';

class PageState {
	query = $state<string>('');
	street = $state<string | null>(null);
	species = $state<string | null>(null);
	age = $state<number>(31_536_000); // 1 year

	state = $state<string>('');
	keywords = $state<string>('');

	rebuildQuery = () => {
		const parts = [];

		if (this.keywords) {
			parts.push(this.keywords);
		}

		if (this.street) {
			parts.push(`addr:"${this.street}"`);
		}

		if (this.species) {
			parts.push(`species:"${this.species}"`);
		}

		if (this.state) {
			parts.push(`state:${this.state}`);
		}

		if (this.age !== 31_536_000) {
			parts.push(`age:${this.age}`);
		}

		this.query = parts.join(' ').trim();
	};

	handleStreetChange = (value: string) => {
		this.street = value.trim() || null;
		this.rebuildQuery();
		console.debug(`Street set to: ${this.street}`);
	};

	handleSpeciesChange = (value: string) => {
		this.species = value.trim() || null;
		this.rebuildQuery();
		console.debug(`Species set to: ${this.species}`);
	};

	handleAgeChange = (value: string) => {
		this.age = parseInt(value, 10);
		this.rebuildQuery();
		console.debug(`Age set to: ${this.age}`);
	};

	handleMissingHeightChange = (value: boolean) => {
		mapLayerStore.update((store) => {
			store.missingHeight = value;
			return store;
		});
	};

	handleMissingDiameterChange = (value: boolean) => {
		mapLayerStore.update((store) => {
			store.missingDiameter = value;
			return store;
		});
	};

	handleMissingCircumferenceChange = (value: boolean) => {
		mapLayerStore.update((store) => {
			store.missingCircumference = value;
			return store;
		});
	};

	handleMissingObservationsChange = (value: boolean) => {
		mapLayerStore.update((store) => {
			store.missingObservations = value;
			return store;
		});
	};

	handleMissingPhotosChange = (value: boolean) => {
		mapLayerStore.update((store) => {
			store.missingPhotos = value;
			return store;
		});
	};

	handleStateChange = (value: string) => {
		this.state = value;
		this.rebuildQuery();
	};

	handleInput = (value: string) => {
		this.query = value;
		this.parseQuery(value);
	};

	parseQuery = (query: string) => {
		let remaining = query;

		// Extract tags and update internal state
		const streetMatch = remaining.match(/\baddr:(?:"([^"]+)"|(\S+))/);
		if (streetMatch) {
			this.street = streetMatch[1] || streetMatch[2];
			remaining = remaining.replace(streetMatch[0], '');
		} else {
			this.street = null;
		}

		const speciesMatch = remaining.match(/\bspecies:(?:"([^"]+)"|(\S+))/);
		if (speciesMatch) {
			this.species = speciesMatch[1] || speciesMatch[2];
			remaining = remaining.replace(speciesMatch[0], '');
		} else {
			this.species = null;
		}

		const ageMatch = remaining.match(/\bage:(\d+)/);
		if (ageMatch) {
			this.age = parseInt(ageMatch[1], 10);
			remaining = remaining.replace(ageMatch[0], '');
		} else {
			this.age = 31_536_000;
		}

		const stateMatch = remaining.match(/\bstate:(\w+)/);
		if (stateMatch) {
			const s = stateMatch[1];
			this.state = s === 'healthy' ? 'alive' : s;
			remaining = remaining.replace(stateMatch[0], '');
		} else {
			// Check legacy status keywords
			const legacyStates = ['healthy', 'alive', 'error', 'dead', 'stump', 'gone', 'replaced'];
			let found = false;
			for (const s of legacyStates) {
				const regex = new RegExp(`\\b${s}\\b`, 'i');
				if (regex.test(remaining)) {
					this.state = s;
					remaining = remaining.replace(regex, '');
					found = true;
					break;
				}
			}
			if (!found) this.state = '';
		}

		const noHeightRegex = /\bno:height\b/i;
		if (noHeightRegex.test(remaining)) {
			if (!get(mapLayerStore).missingHeight) {
				mapLayerStore.update((store) => {
					store.missingHeight = true;
					return store;
				});
			}
			remaining = remaining.replace(noHeightRegex, '');
		}

		const noDiameterRegex = /\bno:diameter\b/i;
		if (noDiameterRegex.test(remaining)) {
			if (!get(mapLayerStore).missingDiameter) {
				mapLayerStore.update((store) => {
					store.missingDiameter = true;
					return store;
				});
			}
			remaining = remaining.replace(noDiameterRegex, '');
		}

		const noCircumferenceRegex = /\bno:circumference\b/i;
		if (noCircumferenceRegex.test(remaining)) {
			if (!get(mapLayerStore).missingCircumference) {
				mapLayerStore.update((store) => {
					store.missingCircumference = true;
					return store;
				});
			}
			remaining = remaining.replace(noCircumferenceRegex, '');
		}

		const noObservationsRegex = /\bno:observations\b/i;
		if (noObservationsRegex.test(remaining)) {
			if (!get(mapLayerStore).missingObservations) {
				mapLayerStore.update((store) => {
					store.missingObservations = true;
					return store;
				});
			}
			remaining = remaining.replace(noObservationsRegex, '');
		}

		const noPhotosRegex = /\b(no:photo|noimage|nophoto)\b/i;
		if (noPhotosRegex.test(remaining)) {
			if (!get(mapLayerStore).missingPhotos) {
				mapLayerStore.update((store) => {
					store.missingPhotos = true;
					return store;
				});
			}
			remaining = remaining.replace(noPhotosRegex, '');
		}

		this.keywords = remaining.replace(/\s+/g, ' ').trim();
	};

	handleSearch = (value: string) => {
		goto(routes.searchQuery(value));
	};

	handleSubmit = async (e?: Event) => {
		e?.preventDefault();
		await goto(routes.searchQuery(this.query));
	};

	getAgeQuery = (baseQuery: string) => {
		if (this.age === 31_536_000) {
			return baseQuery;
		}
		return `${baseQuery} age:${this.age}`;
	};
}

export const pageState = new PageState();
