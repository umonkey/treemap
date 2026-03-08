<script lang="ts">
import { isInstallable, pwaStore } from "$lib/stores/pwaStore";

const onClick = () => {
	const event = $pwaStore;
	if (event) {
		event.prompt();
	}
};
</script>

{#if $isInstallable}
	<button class="button" onclick={onClick}>Install application</button>
{/if}
