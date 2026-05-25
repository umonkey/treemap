<script lang="ts">
	import Button from '$lib/ui/button/Button.svelte';
	import { componentState } from './CheckUpdatesButton.svelte.ts';
	import { onMount, onDestroy } from 'svelte';

	onMount(() => {
		componentState.init();
	});

	onDestroy(() => {
		componentState.destroy();
	});
</script>

<Button type="secondary" onClick={componentState.checkUpdates} disabled={componentState.isChecking}>
	{componentState.isChecking ? 'Checking...' : 'Check for updates'}
</Button>
