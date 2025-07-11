import { apiClient } from '$lib/api';
import { writable, get } from 'svelte/store';

export const hooks = () => {
	const address = writable<string | null>(null);

	const handleStreetChange = (value: string) => {
		address.set(value.trim() || null);
	};

	const handleSubmit = () => {
		const addr = get(address);

		if (addr) {
			apiClient.getStreetReport(addr).then((response) => {
				console.debug('RES', response);
			});
		}
	};

	return { handleStreetChange, handleSubmit };
};
