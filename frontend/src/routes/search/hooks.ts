import { writable, get } from 'svelte/store';
import { routes, goto } from '$lib/routes';

export const hooks = () => {
	const street = writable<string | null>(null);
	const species = writable<string | null>(null);

	const handleStreetChange = (value: string) => {
		street.set(value.trim() || null);
		console.debug(`Street set to: ${get(street)}`);
	};

	const handleSpeciesChange = (value: string) => {
		species.set(value.trim() || null);
		console.debug(`Species set to: ${get(street)}`);
	};

	const handleSubmit = () => {
		const parts = [];

		if (get(street)) {
			parts.push(`addr:"${get(street)}"`);
		}

		if (get(species)) {
			parts.push(`species:"${get(species)}"`);
		}

		const query = parts.join(' ');
		goto(routes.searchQuery(query));
	};

	return { handleStreetChange, handleSpeciesChange, handleSubmit };
};
