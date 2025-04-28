<script lang="ts">
	import MapPreview from '$lib/components/map/MapPreview.svelte';
	import { Header, Map } from '$lib/ui';
	import { locale } from '$lib/locale';
	import { goto, routes } from '$lib/routes';
	import { mapCenter, mapStore, mapZoom, setLastTree } from '$lib/stores/mapStore';
	import { isMapperMode } from '$lib/stores/modeStore';
	import type { ITree } from '$lib/types';
	import { mapState } from './hooks';

	const { data } = $props();
	const searchQuery = data.searchQuery;
	const selectedTree = $derived(data.preview);

	const { marker, reload } = mapState();

	// Components cannot see store updates directly,
	// so we need to use $derived to get the values.
	const markerPosition = $derived($marker);

	const title = searchQuery ? locale.mapTitleQuery(searchQuery) : locale.mapTitle();

	// This is called when a user clicks a tree on the map.
	// We need to show a preview, for this we redirect to the page
	// with the right arguments.
	const onChange = (tree: ITree) => {
		setLastTree(tree.id);
		goto(routes.mapPreview(tree.id));
	};

	const onClosePreview = () => {
		setLastTree(null);
		goto(routes.map());
	};

	// The user moves/pans the map.  Save the new center and zoom.
	const onMove = (center: number[], zoom: number) => {
		mapStore.update((state) => ({
			...state,
			center,
			zoom
		}));
	};

	$effect(() => reload(data.preview));

	$effect(() => {
		const ll = $marker;
		console.debug(`[map] map page marker=${ll}`);
	});
</script>

<svelte:head>
	<title>{locale.mapTitle()}</title>
</svelte:head>

<Header {title} />

<div class="mapContainer">
	<Map
		center={$mapCenter}
		marker={markerPosition}
		zoom={$mapZoom}
		{onChange}
		{onMove}
		{searchQuery}
		crosshair={$isMapperMode}
		canAdd={$isMapperMode}
	/>
	<MapPreview tree={selectedTree} onClose={onClosePreview} />
</div>

<style>
	.mapContainer {
		height: calc(100dvh - 41px);
		z-index: var(--z-map-preview);
		position: relative;
	}

	@media (max-width: 480px) {
		.mapContainer {
			height: calc(100dvh - 91px);
		}
	}
</style>
