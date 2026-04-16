import { routes } from '$lib/routes';
import { showError } from '$lib/errors';

export const handleShareTree = async (id: string) => {
	if (!navigator.share) {
		showError("Your browser doesn't support sharing.");
		return;
	}

	try {
		await navigator.share({
			title: document.title,
			url: routes.treeDetails(id)
		});
	} catch (e) {
		console.error('Error sharing a tree.', e);
		showError(`Error sharing this page: ${e}`);
	}
};
