<script lang="ts">
	import { isInstallable, pwaStore } from '$lib/stores/pwaStore';
	import { locale } from '$lib/locale';
	import Button from '$lib/ui/button/Button.svelte';

	const onClick = () => {
		const event = $pwaStore;
		if (event) {
			event.prompt();
		}
	};
</script>

{#if $isInstallable}
	<Button {onClick} type="button">{locale.sideInstallApp()}</Button>
{/if}
