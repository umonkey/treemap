<script lang="ts">
	import { apiClient } from '$lib/api';
	import { goto } from '$app/navigation';
	import { routes } from '$lib/routes';
	import { toast } from '@zerodevx/svelte-toast';
	import { formatSpecies } from '$lib/utils/trees';
	import { locale } from '$lib/locale';

	import Header from '$lib/components/tree/Header.svelte';
	import Tabs from '$lib/components/tree/Tabs.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import TreeMenu from '$lib/components/tree/Menu.svelte';
	import CommentForm from '$lib/components/forms/CommentForm.svelte';
	import Comment from '$lib/components/tree/Comment.svelte';

	const { data } = $props();
	const tree = data.tree;
	const comments = data.comments;

	const onSubmit = (message: string) => {
		apiClient
			.addComment(tree.id, message)
			.then((res) => {
				if (res.status >= 200 && res.status < 300) {
					toast.push('Comment added.');
					goto(routes.treeDetails(tree.id));
				} else {
					console.info(`Error ${res.status} adding a comment.`);
					toast.push('Error adding comment.');
				}
			})
			.catch((e) => {
				console.error('Exception while adding a comment.', e);
				toast.push('Error adding comment.');
			});
	};
</script>

<svelte:head>
	<title>{formatSpecies(tree.species)} - Details</title>
</svelte:head>

<Header title="Tree" />
<Title title={formatSpecies(tree.species)} address={tree.address} />
<Tabs tree={tree.id} active="comments" />
<TreeMenu id={tree.id} />

<div class="container">
	{#if comments.length > 0}
		{#each comments as comment}
			<Comment {comment} />
		{/each}
	{:else}
		<p>{locale.noComments()}</p>
	{/if}

	<CommentForm {onSubmit} />
</div>

<style>
	.container {
		padding: 0 var(--gap);
	}
</style>
