<script lang="ts">
	import { getContext } from 'svelte';
	import { page } from '$app/state';
	import Observations from '$lib/components/observation/Observations.svelte';
	import Gallery from '$lib/components/photos/Gallery.svelte';
	import Comment from '$lib/components/tree/Comment.svelte';
	import { locale } from '$lib/locale';
	import CommentForm from './components/CommentForm.svelte';
	import Description from './components/Description.svelte';
	import { pageState } from './hooks.svelte.ts';
	import { TREE_CONTEXT_KEY, type TreeLayoutState } from './layout.svelte.ts';

	const layoutState = getContext<TreeLayoutState>(TREE_CONTEXT_KEY);
	const tree = $derived(layoutState.tree);

	const id = $derived(page.params.id as string);

	$effect(() => {
		pageState.reload(id);
	});
</script>

{#if tree}
	<Gallery id={tree.id} />

	<Observations observation={pageState.observation || null} />
	<Description text={tree.notes} />

	<div id="comments" class="comments">
		{#if pageState.comments.length > 0}
			{#each pageState.comments as comment}
				<Comment {comment} />
			{/each}
		{:else}
			<p class="empty">{locale.noComments()}</p>
		{/if}

		<CommentForm onSubmit={pageState.handleSubmitComment} />
	</div>
{/if}

<style>
	.comments {
		border-top: 1px solid var(--sep-color);
		margin-top: var(--gap);
	}

	.empty {
		padding: var(--gap) 0;
	}
</style>
