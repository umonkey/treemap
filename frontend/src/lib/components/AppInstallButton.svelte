<script lang="ts">
	import { pwaStore, isInstallable } from '$lib/stores/pwaStore';
	import { onMount } from 'svelte';

	const onClick = () => {
		const event = $pwaStore;
		event.prompt();
	};

	onMount(() => {
		window.addEventListener('beforeinstallprompt', (e) => {
			e.preventDefault();
			pwaStore.update(() => e);
			console.debug('[pwa] Install event stored.');
		});
	});
</script>

{#if $isInstallable}
	<button class="button" onclick={onClick}>Install application</button>
{/if}
