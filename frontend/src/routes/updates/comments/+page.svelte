<script lang="ts">
	import TreeListPreview from '$lib/components/TreeListPreview.svelte';
	import Comment from '$lib/components/tree/Comment.svelte';
	import Tabs from '$lib/components/updates/Tabs.svelte';
	import Dialog from '$lib/components/layout/Dialog.svelte';

	const { data } = $props();
	const { comments } = data;
</script>

<Dialog title="Recent Comments">
	<Tabs active="comments" />

	{#each comments as comment}
		<TreeListPreview id={comment.tree_id.toString()} />
		<Comment {comment} />
	{/each}
</Dialog>
