// Loads data required by the species editor, performs updates.

import { updateTreeSpecies } from '$lib/api/trees';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';

export class SpeciesState {
	id = $state<string | undefined>(undefined);
	value = $state<string>('');
	canSave = $state<boolean>(false);
	saving = $state<boolean>(false);

	save = () => {
		const id = this.id;
		const species = this.value.trim();

		if (id === undefined || species.length === 0) {
			return;
		}

		this.saving = true;

		updateTreeSpecies(id, species)
			.then((res) => {
				if (res.status >= 200 && res.status < 300 && res.data) {
					goto(routes.treeDetails(id));
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
			goto(routes.treeDetails(this.id));
		}
	};

	handleChange = (value: string) => {
		this.value = value;
		this.canSave = value.trim().length > 0;
	};

	reload = (id: string) => {
		this.id = id;
		this.value = '';
		this.canSave = false;
	};
}
