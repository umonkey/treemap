<script lang="ts">
	import { goto } from '$app/navigation';
	import { apiClient } from '$lib/api';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { formatSpecies } from '$lib/utils/trees';
	import { toast } from '@zerodevx/svelte-toast';

	import { CommentForm, NarrowPage, TreeContextMenu, TreeTabs } from '$lib/ui';
	import Comment from '$lib/components/tree/Comment.svelte';
	import Title from '$lib/components/tree/Title.svelte';

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

<NarrowPage title={formatSpecies(tree.species)} nopadding>
	<Title title={formatSpecies(tree.species)} address={tree.address} />
	<TreeTabs tree={tree.id} active="comments" comment_count={tree.comment_count} />
	<TreeContextMenu id={tree.id} />

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
</NarrowPage>

<style>
	.container {
		padding: 0 var(--gap);
	}
</style>
