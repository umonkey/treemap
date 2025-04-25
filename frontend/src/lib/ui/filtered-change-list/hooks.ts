import { formatDate } from '$lib/utils/strings';
import type { IChange } from '$lib/types';
import { getUser } from '$lib/stores/userStore';
import { get } from 'svelte/store';

type Record = {
	date: string;
	value: string;
	author: string;
};

export const filter = (changes: IChange[], name: string) => {
	return changes.filter((change) => change.name === name);
};

export const format = (changes: IChange[], name: string): Record[] => {
	const filtered = filter(changes, name);

	return filtered.map(
		(change) =>
			({
				date: formatDate(change.added_at),
				value: change.value,
				author: get(getUser)(change.added_by)?.name ?? '(unknown user)'
			}) as Record
	);
};
