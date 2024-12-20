<script>
	import Header from '$lib/components/tree/Header.svelte';
	import Comment from '$lib/components/tree/Comment.svelte';
	import TreeListPreview from '$lib/components/TreeListPreview.svelte';

	const { data } = $props();
	const { comments } = data;
</script>

<Header title="Recent Comments" />

<div class="padded">
	{#each comments as comment}
		<TreeListPreview id={comment.tree_id} />
		<Comment {comment} />
	{/each}
</div>
