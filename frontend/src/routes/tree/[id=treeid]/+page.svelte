<script lang="ts">
	import { locale } from '$lib/locale';
	import { pageState } from './hooks.svelte.ts';
	import { formatSpecies } from '$lib/utils/trees';

	import Actions from '$lib/components/tree/Actions.svelte';
	import Comment from '$lib/components/tree/Comment.svelte';
	import Description from '$lib/components/tree/Description.svelte';
	import Observations from '$lib/components/observation/Observations.svelte';
	import Properties from '$lib/components/tree/Properties.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import Gallery from '$lib/components/photos/Gallery.svelte';
	import TreeTabs from '$lib/components/tree/TreeTabs.svelte';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { CommentForm } from '$lib/ui';

	const { data } = $props();

	$effect(() => {
		pageState.reload(data.id);
	});
</script>

<Dialog title={formatSpecies(pageState.tree?.species || null)} nopadding>
	{#if pageState.tree}
		<div>
			<Title
				id={pageState.tree.id}
				title={pageState.tree.species}
				address={pageState.tree.address}
				padded
			/>

			<TreeTabs tree={pageState.tree.id} active="details" />

			<Gallery id={pageState.tree.id} />
			<Actions tree={pageState.tree} />
			<Properties tree={pageState.tree} />
			<Observations observation={pageState.observation || null} />
			<Description text={pageState.tree.notes} />

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
		</div>
	{:else}
		<p>Loading...</p>
	{/if}
</Dialog>

<style>
	.comments {
		padding: 0 var(--gap);
		border-top: 1px solid var(--sep-color);
		margin-top: var(--gap);
	}

	.empty {
		padding: var(--gap) 0;
	}
</style>
