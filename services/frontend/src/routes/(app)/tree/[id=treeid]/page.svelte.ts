import { getObservations } from '$lib/api/observations';
import type { IObservation } from '$lib/types';

export class TreePageState {
	observations = $state<IObservation | null>(null);

	reload = (id: string) => {
		console.debug(`Reloading preview for tree ${id}`);

		getObservations(id).then((res) => {
			if (res.status === 200 && res.data) {
				this.observations = res.data;
			} else {
				this.observations = null;
			}
		});
	};
}
