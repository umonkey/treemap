<script lang="ts">
	import { page } from '$app/stores';
	import { pageState } from './page.svelte.ts';

	const q = $derived($page.url.searchParams.get('q'));
	const preview = $derived($page.url.searchParams.get('preview'));

	$effect(() => pageState.updateSearch(q));
	$effect(() => pageState.updatePreview(preview));
</script>

<svelte:head>
	<title>Trees of Yerevan</title>
</svelte:head>
