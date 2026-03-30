<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { apiClient } from '$lib/api';
	import { locale } from '$lib/locale';
	import { showError } from '$lib/errors';

	import Actions from '$lib/components/tree/Actions.svelte';
	import Comment from '$lib/components/tree/Comment.svelte';
	import Description from '$lib/components/tree/Description.svelte';
	import Links from '$lib/components/tree/Links.svelte';
	import Observations from '$lib/components/observation/Observations.svelte';
	import Properties from '$lib/components/tree/Properties.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import Gallery from '$lib/components/photos/Gallery.svelte';
	import TreeTabs from '$lib/components/tree/TreeTabs.svelte';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { CommentForm } from '$lib/ui';
	import { onMount } from 'svelte';

	const { data } = $props();
	const tree = $derived(data.tree);
	const comments = $derived(data.comments);
	const observation = $derived(data.observation);

	// Save last active tree.
	onMount(() => {
		console.debug('[map] Last active tree set to', tree.id);
	});

	const onSubmit = (message: string) => {
		apiClient
			.addComment(tree.id, message)
			.then((res) => {
				if (res.status >= 200 && res.status < 300) {
					invalidateAll();
				} else {
					console.info(`Error ${res.status} adding a comment.`);
					showError(locale.toastErrorAddingComment());
				}
			})
			.catch((e) => {
				console.error('Exception while adding a comment.', e);
				showError(locale.toastErrorAddingComment());
			});
	};
</script>

<Dialog title={tree.species}>
	<Title id={tree.id} title={tree.species} address={tree.address} padded />

	<TreeTabs tree={tree.id} active="details" />

	<Gallery id={tree.id} />
	<Actions {tree} />
	<Properties {tree} />
	<Observations {observation} />
	<Links {tree} />
	<Description text={tree.notes} />

	<div id="comments" class="comments">
		{#if comments.length > 0}
			{#each comments as comment}
				<Comment {comment} />
			{/each}
		{:else}
			<p class="empty">{locale.noComments()}</p>
		{/if}

		<CommentForm {onSubmit} />
	</div>
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
