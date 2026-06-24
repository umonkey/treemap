<script lang="ts">
	import TreeListPreview from '$lib/components/layout/TreeListPreview.svelte';
	import Comment from '$lib/components/tree/Comment.svelte';
	import { pageState } from './page.svelte.ts';

	$effect(() => {
		pageState.load();
	});
</script>

{#if pageState.loading && pageState.comments.length === 0}
	<p>Loading comments...</p>
{:else if pageState.error}
	<p>Error loading comments.</p>
{:else}
	{#each pageState.comments as comment (comment.id)}
		<TreeListPreview id={comment.tree_id.toString()} />
		<Comment {comment} />
	{/each}
{/if}
