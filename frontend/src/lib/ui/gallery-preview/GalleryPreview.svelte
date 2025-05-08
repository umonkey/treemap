<script lang="ts">
	// This is the gallery preview shown on the map when you click a tree.
	// It shows the first 3 images of the tree, if any, or a default image.
	//
	// In the mapper mode, clicking on the default image redirects to the
	// adding photo page.
	//
	// By default, trees returned in batches don't contain files, so we have to
	// request tree details again.

	import { isMapperMode } from '$lib/stores/modeStore';
	import { hooks } from './hooks';
	import GalleryPreviewDisplay from './GalleryPreviewDisplay.svelte';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, error, images, reload } = hooks();

	$effect(() => reload(id));
</script>

<GalleryPreviewDisplay
	loading={$loading}
	error={$error}
	mapper={$isMapperMode}
	tree_id={id}
	images={$images}
/>
