// Loads data required by the crown editor, performs updates.

import { apiClient } from '$lib/api';
import { goto, routes } from '$lib/routes';
import { showError } from '$lib/errors';

class GirthState {
	id = $state<string | undefined>(undefined);
	value = $state<number>(0);
	canSave = $state<boolean>(false);

	handleChange = (value: number) => {
		this.value = value;
		this.canSave = value > 0;
	};

	reload = (id: string) => {
		this.id = id;
		this.value = 0;
		this.canSave = false;
	};

	save = () => {
		const id = this.id;

		if (!id || !this.value) {
			return;
		}

		this.canSave = false;

		apiClient
			.updateTreeCircumference(id, this.value)
			.then((res) => {
				if (res.status >= 200 && res.status < 300 && res.data) {
					console.debug(`[crown editor] Tree ${id} updated.`);
					goto(routes.mapPreview(id));
				} else if (res.error) {
					showError(res.error.description);
					console.error(`[crown editor] Failed to update tree ${id}: ${res.error.description}.`);
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
}

export const girthState = new GirthState();
