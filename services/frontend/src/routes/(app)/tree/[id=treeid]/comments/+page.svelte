<script lang="ts">
	import { page } from '$app/state';
	import CommentForm from './CommentForm.svelte';
	import CommentList from './CommentList.svelte';
	import { pageState } from './page.svelte.ts';

	const id = $derived(page.params.id as string);

	$effect(() => {
		pageState.reload(id);
	});
</script>

<CommentList comments={pageState.comments} />

<CommentForm onSubmit={pageState.handleSubmitComment} />
