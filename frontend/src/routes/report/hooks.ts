import { getStreetReport } from '$lib/api/streets';
import { goto, routes } from '$lib/routes';
import type { StreetReport } from '$lib/types';
import { writable } from 'svelte/store';

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

		getStreetReport(address).then(({ status, data: d }) => {
			if (status === 200 && d) {
				report.set(d);
			} else {
				report.set(null);
			}
		});
	};

	return { handleStreetChange, reload, report };
};
