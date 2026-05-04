<script lang="ts">
	import { onMount } from 'svelte';
	import { wakeLockState } from './state.svelte';
	import { releaseWakeLock } from './utils';

	onMount(() => {
		const visibilityHandler = () => wakeLockState.handleVisibilityChange();
		document.addEventListener('visibilitychange', visibilityHandler);

		return () => {
			document.removeEventListener('visibilitychange', visibilityHandler);
			releaseWakeLock();
		};
	});

	$effect(() => {
		wakeLockState.update();
	});
</script>
