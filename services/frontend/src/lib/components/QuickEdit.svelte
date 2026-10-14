<script lang="ts">
	import type { Snippet } from 'svelte';
	import { goto } from '$lib/routes';
	import { longtap } from '$lib/utils/longtap';

	const { url, children }: { url: string; children: Snippet } = $props();

	// Guards against a touch double-tap firing both `dblclick` and the long tap.
	let lastOpen = 0;

	const open = () => {
		const now = Date.now();
		if (now - lastOpen < 500) return;
		lastOpen = now;
		document.getSelection()?.empty();
		goto(url);
	};
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<span class="quick-edit" use:longtap={open} ondblclick={open}>
	{@render children()}
</span>

<style>
	.quick-edit {
		cursor: pointer;
		-webkit-touch-callout: none;
		user-select: none;
	}
</style>
