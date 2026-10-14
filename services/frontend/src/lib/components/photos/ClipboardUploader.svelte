<script lang="ts">
	import { ClipboardUploader } from './ClipboardUploader.svelte.ts';

	const { treeId }: { treeId: string } = $props();

	const state = new ClipboardUploader();
</script>

<svelte:window onpaste={(event) => state.handlePaste(treeId, event)} />
