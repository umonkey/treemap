<script lang="ts">
	/**
	 * This is the pop-up menu shown after clicking the (...) icon on a tree page.
	 *
	 * This component should be placed somewhere on the page, it will be centered
	 * in the container, overlaying that container with a blurry background.
	 */
	import { menuState } from '$lib/stores/treeMenu';
	import TreeContextMenuDisplay from './TreeContextMenuDisplay.svelte';

	const { id } = $props<{
		id: string;
	}>();

	const onCloseMenu = () => {
		menuState.set(false);
	};
</script>

<TreeContextMenuDisplay {id} visible={$menuState} onClose={onCloseMenu} />
