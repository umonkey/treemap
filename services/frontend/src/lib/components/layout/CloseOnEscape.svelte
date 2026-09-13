<script lang="ts">
	const { onClose }: { onClose: () => void } = $props();

	const handleKeyDown = (event: KeyboardEvent) => {
		if (event.key === 'Escape') {
			onClose();
		}
	};
</script>

<svelte:window onkeydown={handleKeyDown} />
