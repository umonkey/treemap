import { routes } from '$lib/routes';
import { showError, showInfo } from '$lib/errors';

/**
 * The user dismissed the native share sheet. WebKit has shipped this both as a
 * `DOMException` and as a plain `Error`, so the name is the reliable check.
 */
const isShareCancelled = (e: unknown) => (e as { name?: unknown } | null)?.name === 'AbortError';

export const handleShareTree = async (id: string) => {
	if (!navigator.share) {
		showInfo("Your browser doesn't support sharing.");
		return;
	}

	try {
		await navigator.share({
			title: document.title,
			url: routes.treeDetails(id)
		});
	} catch (e) {
		if (isShareCancelled(e)) return;

		console.error('Error sharing a tree.', e);
		showError(`Error sharing this page: ${e}`);
	}
};
