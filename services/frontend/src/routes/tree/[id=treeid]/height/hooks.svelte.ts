// Loads data required by the height editor, performs updates.

import { updateTreeHeight } from '$lib/api/trees';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';

class HeightState {
	id = $state<string | undefined>(undefined);
	value = $state<number>(0);
	canSave = $state<boolean>(false);
	saving = $state<boolean>(false);

	save = () => {
		const id = this.id;

		if (id === undefined || this.value <= 0) {
			return;
		}

		this.saving = true;

		updateTreeHeight(id, this.value)
			.then((res) => {
				if (res.status >= 200 && res.status < 300 && res.data) {
					goto(routes.mapPreview(id));
				} else if (res.error) {
					showError(res.error.description);
				}
			})
			.finally(() => {
				this.saving = false;
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
