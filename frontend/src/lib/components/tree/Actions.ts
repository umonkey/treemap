import { getTreeActors } from '$lib/api/trees';
import { type IUser } from '$lib/types';
import { writable } from 'svelte/store';

export const hooks = () => {
	const actors = writable<IUser[]>([]);

	const reload = (id: string) => {
		getTreeActors(id).then(({ status, data }) => {
			if (status === 200 && data?.users) {
				actors.set(data.users);
			} else {
				actors.set([]);
			}
		});
	};

	return { actors, reload };
};
