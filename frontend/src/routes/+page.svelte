<script lang="ts">
	import { page } from '$app/stores';
	import { searchStore } from '$lib/stores/searchStore';
	import { rewriteHash } from '$lib/utils/rewrite';

	// Redirect old urls from v1.
	rewriteHash($page.url.hash);

	$effect(() => {
		const q = $page.url.searchParams.get('q');
		if (q) {
			console.debug(`[Home] Extracted q from URL: ${q}`);
			searchStore.set(q);
		}
	});
</script>

<svelte:head>
	<title>Trees of Yerevan</title>
</svelte:head>
