<script lang="ts">
	import { page } from '$app/stores';
	import { pageState } from './page.svelte.ts';

	const q = $derived($page.url.searchParams.get('q'));
	const preview = $derived($page.url.searchParams.get('preview'));

	$effect(() => {
		if (q) {
			pageState.updateSearch(q);
		}
	});

	$effect(() => {
		if (preview) {
			pageState.updatePreview(preview);
		}
	});
</script>

<svelte:head>
	<title>Trees of Yerevan</title>
</svelte:head>
