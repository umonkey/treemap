<script lang="ts">
	import Comment from '$lib/components/tree/Comment.svelte';
	import { locale } from '$lib/locale';
	import type { IComment } from '$lib/types';

	const { comments } = $props<{
		comments: IComment[];
	}>();
</script>

{#if comments.length > 0}
	{#each comments as comment (comment.id)}
		<Comment {comment} />
	{/each}
{:else}
	<p>{locale.noComments()}</p>
{/if}
