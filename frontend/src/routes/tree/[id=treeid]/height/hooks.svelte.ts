// Loads data required by the height editor, performs updates.

import { apiClient } from '$lib/api';
import { goto, routes } from '$lib/routes';
import { showError } from '$lib/errors';

class HeightState {
	id = $state<string | undefined>(undefined);
	value = $state<number>(0);
	canSave = $state<boolean>(false);

	save = () => {
		const id = this.id;

		if (id === undefined || this.value <= 0) {
			return;
		}

		this.canSave = false;

		apiClient
			.updateTreeHeight(id, this.value)
			.then((res) => {
				if (res.status >= 200 && res.status < 300 && res.data) {
					goto(routes.mapPreview(id));
				} else if (res.error) {
					showError(res.error.description);
				}
			})
			.finally(() => {
				this.canSave = true;
			});
	};

	close = () => {
		if (this.id) {
			goto(routes.mapPreview(this.id));
		}
	};

	handleChange = (value: number) => {
		this.value = value;
		this.canSave = value > 0;
	};

	reload = (id: string) => {
		this.id = id;
		this.value = 0;
		this.canSave = false;
	};
}

export const heightState = new HeightState();
