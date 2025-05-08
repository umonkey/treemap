<script lang="ts">
	import { isSidebarVisible, mobileSidebarStore } from '$lib/stores/mobileSidebarStore';
	import { mapLastTree } from '$lib/stores/mapStore';
	import { searchStore } from '$lib/stores';
	import MobileSidebarDisplay from './MobileSidebarDisplay.svelte';

	const onClick = () => {
		mobileSidebarStore.update(() => false);
	};
</script>

<MobileSidebarDisplay
	visible={$isSidebarVisible}
	last_tree={$mapLastTree}
	search={$searchStore}
	onClose={onClick}
/>
