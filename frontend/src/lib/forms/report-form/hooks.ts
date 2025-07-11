import { apiClient } from '$lib/api';
import type { StreetReport } from '$lib/types';
import { writable, get } from 'svelte/store';

export const hooks = () => {
	const address = writable<string | null>(null);
	const report = writable<StreetReport | null>(null);

	const handleStreetChange = (value: string) => {
		address.set(value.trim() || null);
	};

	const handleSubmit = () => {
		const addr = get(address);

		if (addr) {
			apiClient.getStreetReport(addr).then(({ status, data: d }) => {
				if (status === 200 && d) {
					report.set(d);
				} else {
					report.set(null);
				}
			});
		}
	};

	return { handleStreetChange, handleSubmit, report };
};
