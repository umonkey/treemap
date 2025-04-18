import { goto } from '$app/navigation';
import { routes } from '$lib/routes';

export const rewriteHash = (hash: string | undefined) => {
	if (!hash) {
		return;
	}

	const matches = hash.match(/^#\/tree\/(\d+)/);

	if (!matches) {
		return;
	}

	goto(routes.treeDetails(matches[1]));
};
