import { toast } from '@zerodevx/svelte-toast';
import { routes } from '$lib/routes';

export const handleShareTree = async (id: string) => {
	if (!navigator.share) {
		toast.push("Your browser doesn't support sharing.");
		return;
	}

	try {
		await navigator.share({
			title: document.title,
			url: routes.treeDetails(id)
		});
	} catch (e) {
		console.error('Error sharing a tree.', e);
		toast.push('Error sharing this page.');
	}
};
