<script lang="ts">
	import { page } from '$app/stores';
	import { previewState } from '$lib/components/layout/MapPreview.svelte.ts';

	const id = $derived($page.params.id as string);

	$effect(() => {
		if (id) {
			previewState.reload(id);
		}

		return () => {
			previewState.clear();
		};
	});
</script>

<svelte:head>
	<title>Tree {id} - Trees of Yerevan</title>
</svelte:head>
