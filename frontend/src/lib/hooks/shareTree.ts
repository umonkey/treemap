import { toast } from '@zerodevx/svelte-toast';

export const handleShareTree = async () => {
	if (!navigator.share) {
		toast.push("Your browser doesn't support sharing.");
		return;
	}

	try {
		await navigator.share({
			title: document.title,
			url: window.location.href
		});
	} catch (e) {
		console.error('Error sharing a tree.', e);
		toast.push('Error sharing this page.');
	}
};
