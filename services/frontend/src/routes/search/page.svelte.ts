import { goto, routes } from '$lib/routes';

class PageState {
	query = $state<string>('');
	street = $state<string | null>(null);
	species = $state<string | null>(null);
	age = $state<number>(31_536_000); // 1 year

	noHeight = $state<boolean>(false);
	noCanopy = $state<boolean>(false);
	noCircumference = $state<boolean>(false);
	noObservations = $state<boolean>(false);
	noPhotos = $state<boolean>(false);
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

		if (this.noHeight) parts.push('no:height');
		if (this.noCanopy) parts.push('no:diameter');
		if (this.noCircumference) parts.push('no:circumference');
		if (this.noObservations) parts.push('no:observations');
		if (this.noPhotos) parts.push('no:photo');

		if (this.state) {
			if (this.state === 'healthy') {
				parts.push('state:alive');
			} else {
				parts.push(`state:${this.state}`);
			}
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

	handleNoHeightChange = (value: boolean) => {
		this.noHeight = value;
		this.rebuildQuery();
	};

	handleNoCanopyChange = (value: boolean) => {
		this.noCanopy = value;
		this.rebuildQuery();
	};

	handleNoCircumferenceChange = (value: boolean) => {
		this.noCircumference = value;
		this.rebuildQuery();
	};

	handleNoObservationsChange = (value: boolean) => {
		this.noObservations = value;
		this.rebuildQuery();
	};

	handleNoPhotosChange = (value: boolean) => {
		this.noPhotos = value;
		this.rebuildQuery();
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
			this.state = s === 'alive' ? 'healthy' : s;
			remaining = remaining.replace(stateMatch[0], '');
		} else {
			// Check legacy status keywords
			const legacyStates = ['healthy', 'dead', 'stump', 'gone', 'replaced'];
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
		this.noHeight = noHeightRegex.test(remaining);
		if (this.noHeight) remaining = remaining.replace(noHeightRegex, '');

		const noCanopyRegex = /\bno:diameter\b/i;
		this.noCanopy = noCanopyRegex.test(remaining);
		if (this.noCanopy) remaining = remaining.replace(noCanopyRegex, '');

		const noCircumferenceRegex = /\bno:circumference\b/i;
		this.noCircumference = noCircumferenceRegex.test(remaining);
		if (this.noCircumference) remaining = remaining.replace(noCircumferenceRegex, '');

		const noObservationsRegex = /\bno:observations\b/i;
		this.noObservations = noObservationsRegex.test(remaining);
		if (this.noObservations) remaining = remaining.replace(noObservationsRegex, '');

		const noPhotosRegex = /\b(no:photo|noimage|nophoto)\b/i;
		this.noPhotos = noPhotosRegex.test(remaining);
		if (this.noPhotos) remaining = remaining.replace(noPhotosRegex, '');

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
