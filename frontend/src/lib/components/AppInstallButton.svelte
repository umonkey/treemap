<script lang="ts">
	import { isInstallable, pwaStore } from '$lib/stores/pwaStore';
	import Button from '$lib/ui/button/Button.svelte';

	const onClick = () => {
		const event = $pwaStore;
		if (event) {
			event.prompt();
		}
	};
</script>

{#if $isInstallable}
	<Button {onClick} type="button">Install Application</Button>
{/if}
