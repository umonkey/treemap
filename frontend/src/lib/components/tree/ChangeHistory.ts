import { formatDate } from '$lib/utils/strings';
import type { IChange } from '$lib/types';
import { addUsers, getUser } from '$lib/stores/userStore';
import { get, writable } from 'svelte/store';
import { apiClient } from '$lib/api';
import PLACEHOLDER from '$lib/assets/cat.jpeg';

type Record = {
	date: string;
	value: string;
	author: string;
	image: string;
};

export const filter = (changes: IChange[], name: string) => {
	return changes.filter((change) => change.name === name);
};

export const format = (changes: IChange[]): Record[] => {
	return changes.map((change) => {
		const user = get(getUser)(change.added_by);

		return {
			date: formatDate(change.added_at),
			value: change.value,
			author: user?.name ?? '(unknown user)',
			image: user?.picture ?? PLACEHOLDER
		} as Record;
	});
};

export const load = (id: string, propName: string, limit: number) => {
	const loading = writable<boolean>(true);
	const error = writable<boolean>(false);
	const history = writable<IChange[]>([]);

	apiClient
		.getTreeHistory(id)
		.then((res) => {
			if (res.status >= 200 && res.status < 400 && res.data) {
				history.set(filter(res.data.props, propName).slice(0, limit));
				addUsers(res.data.users);
			} else {
				error.set(true);
			}
		})
		.finally(() => {
			loading.set(false);
		});

	return { loading, error, history };
};
