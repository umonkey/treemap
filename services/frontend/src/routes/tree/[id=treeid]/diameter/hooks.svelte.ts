// Loads data required by the crown editor, performs updates.

import { updateTreeDiameter } from '$lib/api/trees';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';

class CrownState {
	id = $state<string | undefined>(undefined);
	value = $state<number>(0);
	canSave = $state<boolean>(false);
	saving = $state<boolean>(false);

	save = () => {
		const id = this.id;

		if (!id || this.value <= 0) {
			return;
		}

		this.saving = true;

		updateTreeDiameter(id, this.value)
			.then((res) => {
				if (res.status >= 200 && res.status < 300 && res.data) {
					console.debug(`[crown editor] Tree ${id} updated.`);
					goto(routes.mapPreview(id));
				} else if (res.error) {
					showError(res.error.description);
					console.error(
						`[crown editor] Failed to update tree ${id}: ${res.error.description}.`,
						res.error
					);
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

export const crownState = new CrownState();
