<script lang="ts">
	import { isSidebarVisible, mobileSidebarStore } from '$lib/stores/mobileSidebarStore';
	import { searchStore } from '$lib/stores';
	import MobileSidebarDisplay from './MobileSidebarDisplay.svelte';

	const onClick = () => {
		mobileSidebarStore.update(() => false);
	};
</script>

<MobileSidebarDisplay visible={$isSidebarVisible} search={$searchStore} onClose={onClick} />
