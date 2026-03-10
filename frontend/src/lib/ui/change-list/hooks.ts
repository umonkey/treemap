import type { IChange } from '$lib/types';
import { get } from 'svelte/store';
import { getUser } from '$lib/stores/userStore';
import { formatDate } from '$lib/utils/strings';
import PLACEHOLDER from '$lib/assets/cat.jpeg';

type Response = {
	header: string;
	body: string;
	picture: string;
};

export const format = (changes: IChange[]): Response[] => {
	return changes.map((change) => {
		const date = formatDate(change.added_at);
		const user = get(getUser)(change.added_by);
		const userName = user?.name ?? '(unknown user)';

		return {
			header: `${date}, ${userName}:`,
			body: `${change.name} → ${change.value}`,
			picture: user.picture ?? PLACEHOLDER
		} as Response;
	});
};
