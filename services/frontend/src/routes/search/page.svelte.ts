import { goto, routes } from '$lib/routes';

class PageState {
	query = $state<string>('');
	street = $state<string | null>(null);
	species = $state<string | null>(null);

	handleStreetChange = (value: string) => {
		this.street = value.trim() || null;
		console.debug(`Street set to: ${this.street}`);
	};

	handleSpeciesChange = (value: string) => {
		this.species = value.trim() || null;
		console.debug(`Species set to: ${this.species}`);
	};

	handleInput = (value: string) => {
		this.query = value;
	};

	handleSearch = (value: string) => {
		goto(routes.searchQuery(value));
	};

	handleSubmit = async (e?: Event) => {
		e?.preventDefault();
		const parts = [];

		if (this.street) {
			parts.push(`addr:"${this.street}"`);
		}

		if (this.species) {
			parts.push(`species:"${this.species}"`);
		}

		const query = parts.join(' ');

		await goto(routes.searchQuery(query));
	};
}

export const pageState = new PageState();
