import { addWater } from '$lib/api/water';
import { mapBus } from '$lib/buses/mapBus';
import { mapState } from '$lib/components/map/MapLibre.svelte.ts';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';

class PageState {
	saving = $state<boolean>(false);

	private save = async (): Promise<string | null> => {
		this.saving = true;

		try {
			const res = await addWater(mapState.center.lat, mapState.center.lng);
			if (res.status >= 200 && res.status < 400 && res.data) {
				const id = res.data.id;
				if (typeof navigator !== 'undefined' && navigator.vibrate) {
					navigator.vibrate(50);
				}
				return id;
			} else {
				console.error(`Error ${res.status} adding water source.`, res);
				showError(res.error?.description || `Error ${res.status} adding water source.`);
				return null;
			}
		} finally {
			this.saving = false;
			mapBus.emit('reload');
		}
	};

	handleConfirm = async () => {
		const id = await this.save();
		if (id) {
			goto(routes.waterDetails(id));
		}
	};

	handleCancel = () => {
		goto(routes.map());
	};
}

export const pageState = new PageState();
