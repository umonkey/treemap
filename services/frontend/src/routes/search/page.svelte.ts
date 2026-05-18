import { goto, routes } from '$lib/routes';

class PageState {
	query = $state<string>('');
	street = $state<string | null>(null);
	species = $state<string | null>(null);
	age = $state<number>(31_536_000); // 1 year

	handleStreetChange = (value: string) => {
		this.street = value.trim() || null;
		console.debug(`Street set to: ${this.street}`);
	};

	handleSpeciesChange = (value: string) => {
		this.species = value.trim() || null;
		console.debug(`Species set to: ${this.species}`);
	};

	handleAgeChange = (value: string) => {
		this.age = parseInt(value, 10);
		console.debug(`Age set to: ${this.age}`);
	};

	handleInput = (value: string) => {
		this.query = value;
	};

	handleSearch = (value: string) => {
		let query = value;
		if (this.age !== 31_536_000) {
			query += ` age:${this.age}`;
		}
		goto(routes.searchQuery(query));
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

		if (this.age !== 31_536_000) {
			parts.push(`age:${this.age}`);
		}

		const query = parts.join(' ');

		await goto(routes.searchQuery(query));
	};

	getAgeQuery = (baseQuery: string) => {
		if (this.age === 31_536_000) {
			return baseQuery;
		}
		return `${baseQuery} age:${this.age}`;
	};
}

export const pageState = new PageState();
