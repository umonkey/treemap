import { get } from 'svelte/store';
import { addObservations, getObservations } from '$lib/api/observations';
import { getUser as getUserApi } from '$lib/api/users';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import { addUsers, getUser } from '$lib/stores/userStore';
import type { IObservation } from '$lib/types';

class PageState {
	observation = $state<IObservation>({
		id: '',
		tree_id: '',
		created_at: 0,
		created_by: '',
		bark_damage: false,
		dry_branches: false,
		leaking: false,
		root_damage: false,
		open_roots: false,
		bug_holes: false,
		topping: false,
		fungal_bodies: false,
		vfork: false,
		cavities: false,
		vines: false,
		inclined: false,
		nests: false,
		nesting_boxes: false
	});

	loading = $state(true);
	saving = $state(false);
	treeId = $state('');

	reload = async (id: string) => {
		if (this.treeId === id) return;
		this.treeId = id;
		this.loading = true;

		// Reset observation to default values.
		this.observation = {
			id: '',
			tree_id: id,
			created_at: 0,
			created_by: '',
			bark_damage: false,
			dry_branches: false,
			leaking: false,
			root_damage: false,
			open_roots: false,
			bug_holes: false,
			topping: false,
			fungal_bodies: false,
			vfork: false,
			cavities: false,
			vines: false,
			inclined: false,
			nests: false,
			nesting_boxes: false
		};

		try {
			const response = await getObservations(id);
			if (response.data) {
				this.observation = { ...this.observation, ...response.data };

				if (
					this.observation.created_by &&
					this.observation.created_by !== '0' &&
					!get(getUser)(this.observation.created_by)
				) {
					getUserApi(this.observation.created_by).then((res) => {
						if (res.data) {
							addUsers([res.data.user]);
						}
					});
				}
			}
		} catch (err) {
			console.error('Failed to load observations:', err);
		} finally {
			this.loading = false;
		}
	};

	handleSubmit = async () => {
		this.saving = true;
		try {
			const response = await addObservations(this.observation);
			if (response.error) {
				showError(`Error ${response.status} adding observations: ${response.error.description}`);
			} else {
				goto(routes.mapPreview(this.treeId));
			}
		} catch (err) {
			console.error('Failed to save observations:', err);
			showError('Failed to save observations.');
		} finally {
			this.saving = false;
		}
	};

	handleCancel = () => {
		goto(routes.mapPreview(this.treeId));
	};
}

export const pageState = new PageState();
