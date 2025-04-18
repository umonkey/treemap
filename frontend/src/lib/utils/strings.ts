import { getUser } from '$lib/stores/userStore';
import type { ITreeFile } from '$lib/types';
import { get } from 'svelte/store';

export const formatDate = (timestamp: number): string => {
	const date = new Date(timestamp * 1000);

	const day = `00${date.getDate()}`.slice(-2);
	const month = `00${date.getMonth() + 1}`.slice(-2);
	const year = date.getFullYear();

	return `${day}.${month}.${year}`;
};

export const fileAttribution = (file: ITreeFile): string => {
	if (!file.added_at || !file.added_by) {
		return '';
	}

	const user = get(getUser)(file.added_by);

	if (user === undefined) {
		return '';
	}

	const date = formatDate(file.added_at);
	return `${date} by ${user.name}`;
};
