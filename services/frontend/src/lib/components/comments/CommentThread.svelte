<script lang="ts">
	import { locale as appLocale } from '$lib/locale';
	import type { CommentEntityType } from '$lib/types';
	import Comment from './Comment.svelte';
	import CommentForm from './CommentForm.svelte';
	import { CommentThreadState } from './CommentThread.svelte.ts';

	const { entityType, entityId }: { entityType: CommentEntityType; entityId: string } = $props();

	const state = new CommentThreadState();

	$effect(() => {
		state.reload(entityType, entityId);
	});
</script>

{#if state.comments.length > 0}
	{#each state.comments as comment (comment.id)}
		<Comment {comment} />
	{/each}
{:else}
	<p>{appLocale.noComments()}</p>
{/if}

<CommentForm {entityType} {entityId} onSubmitted={state.load} />
