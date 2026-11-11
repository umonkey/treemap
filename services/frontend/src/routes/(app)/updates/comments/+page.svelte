<script lang="ts">
	import Comment from '$lib/components/comments/Comment.svelte';
	import TreeListPreview from '$lib/components/layout/TreeListPreview.svelte';
	import { PageState } from './page.svelte.ts';

	const pageState = new PageState();

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
		{#if comment.entity_type === 'tree'}
			<TreeListPreview id={comment.entity_id} />
		{/if}
		<Comment {comment} />
	{/each}
{/if}
