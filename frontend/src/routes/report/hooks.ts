import { apiClient } from '$lib/api';
import type { StreetReport } from '$lib/types';
import { writable } from 'svelte/store';
import { routes, goto } from '$lib/routes';

export const hooks = () => {
	const report = writable<StreetReport | null>(null);

	const handleStreetChange = (value: string) => {
		goto(routes.streetReport(value));
	};

	const reload = (address: string | null) => {
		if (address === null) {
			report.set(null);
			return;
		}

		apiClient.getStreetReport(address).then(({ status, data: d }) => {
			if (status === 200 && d) {
				report.set(d);
			} else {
				report.set(null);
			}
		});
	};

	return { handleStreetChange, reload, report };
};
