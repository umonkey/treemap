import { get } from 'svelte/store';
import { addObservations, getObservations } from '$lib/api/observations';
import { getUser as getUserApi } from '$lib/api/users';
import { showError } from '$lib/errors';
import { addUsers, getUser } from '$lib/stores/userStore';
import type { IObservation } from '$lib/types';

export type ObservationField = Exclude<
	keyof IObservation,
	'id' | 'tree_id' | 'created_at' | 'created_by'
>;

const emptyObservation = (treeId: string): IObservation => ({
	id: '',
	tree_id: treeId,
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

export class ObservationsPage {
	observation = $state<IObservation>(emptyObservation(''));
	loading = $state(true);
	saving = $state(false);
	treeId = $state('');

	private pending = false;

	reload = async (id: string) => {
		if (this.treeId === id) return;

		this.treeId = id;
		this.loading = true;

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
			// Reset observation to default values.
			this.observation = emptyObservation(id);

			console.error('Failed to load observations:', err);
		} finally {
			this.loading = false;
		}
	};

	/** Persists the observations. Concurrent calls are coalesced into one extra save. */
	save = async () => {
		if (this.saving) {
			this.pending = true;
			return;
		}

		this.saving = true;

		try {
			do {
				this.pending = false;

				const response = await addObservations(this.observation);
				if (response.error) {
					showError(`Error ${response.status} saving observations: ${response.error.description}`);
				} else if (response.data) {
					// Refresh only the metadata: the toggles may have changed again
					// while the request was in flight.
					this.observation.id = response.data.id;
					this.observation.created_at = response.data.created_at;
					this.observation.created_by = response.data.created_by;
				}
			} while (this.pending);
		} catch (err) {
			console.error('Failed to save observations:', err);
			showError('Failed to save observations.');
		} finally {
			this.saving = false;
		}
	};

	handleChange = (field: ObservationField, value: boolean) => {
		this.observation[field] = value;
		void this.save();
	};
}
