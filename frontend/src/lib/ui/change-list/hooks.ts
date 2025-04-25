import type { IChange } from '$lib/types';
import { get } from 'svelte/store';
import { getUser } from '$lib/stores/userStore';
import { formatDate } from '$lib/utils/strings';

type Response = {
	header: string;
	body: string;
};

export const format = (changes: IChange[]): Response[] => {
	return changes.map((change) => {
		const date = formatDate(change.added_at);
		const userName = get(getUser)(change.added_by)?.name ?? '(unknown user)';

		return {
			header: `${date}, ${userName}:`,
			body: `${change.name} → ${change.value}`
		} as Response;
	});
};
