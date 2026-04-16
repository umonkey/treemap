import { getStreetReport } from '$lib/api/streets';
import { goto, routes } from '$lib/routes';
import type { StreetReport } from '$lib/types';

class PageState {
	report = $state<StreetReport | null>(null);

	handleStreetChange = (value: string) => {
		goto(routes.streetReport(value));
	};

	reload = async (address: string | null) => {
		if (address === null) {
			this.report = null;
			return;
		}

		const { status, data: d } = await getStreetReport(address);
		if (status === 200 && d) {
			this.report = d;
		} else {
			this.report = null;
		}
	};
}

export const pageState = new PageState();
