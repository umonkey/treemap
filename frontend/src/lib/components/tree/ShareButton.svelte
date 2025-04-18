<script lang="ts">
	import { ShareIcon } from '$lib/icons';
	import { toast } from '@zerodevx/svelte-toast';

	const onClick = async () => {
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
</script>

<button type="button" onclick={onClick} title="Share this page">
	<ShareIcon />
</button>

<style>
	button {
		background: none;
		border: none;
		color: inherit;
		padding: 0;
		margin: 0;
		cursor: pointer;
	}
</style>
