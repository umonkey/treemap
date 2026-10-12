import { getObservations } from '$lib/api/observations';
import type { IObservation } from '$lib/types';

class PageState {
	observation = $state<IObservation>();

	public reload = (id: string) => {
		this.observation = undefined;

		getObservations(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.observation = res.data;
			}
		});
	};
}

export const pageState = new PageState();
