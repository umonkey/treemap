<script lang="ts">
	import { page } from '$app/state';
	import CommentThread from '$lib/components/comments/CommentThread.svelte';

	const id = $derived(page.params.id as string);
</script>

<CommentThread entityType="panorama" entityId={id} />
