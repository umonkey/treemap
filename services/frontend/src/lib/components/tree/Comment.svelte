<script lang="ts">
	import TrashIcon from '$lib/icons/TrashIcon.svelte';
	import { locale } from '$lib/locale';
	import type { IComment } from '$lib/types';
	import { componentState } from './Comment.svelte.ts';
	import { getUser } from '$lib/stores/userStore';

	const { comment, onDelete }: { comment: IComment; onDelete?: (commentId: string) => void } =
		$props();

	const userName = $derived($getUser(comment.added_by)?.name);
	const dateStr = $derived(componentState.getFormattedDate(comment.added_at));
	const showDelete = $derived(componentState.canDelete(comment.added_by));
</script>

<div class="comment">
	<blockquote>{comment.message}</blockquote>
	<div class="meta">
		{dateStr} · {#if userName}{userName}{:else}{locale.userUnknown()}{/if}
		{#if showDelete}
			·
			<button
				class="delete-comment-btn"
				onclick={() => componentState.handleDelete(comment.tree_id, comment.id, onDelete)}
				disabled={componentState.isDeleting}
				title={locale.commentDelete()}
				aria-label={locale.commentDelete()}
			>
				<TrashIcon />
			</button>
		{/if}
	</div>
</div>

<style>
	blockquote {
		border-left: solid 4px var(--sep-color);
		margin: var(--gap) 0;
		padding: var(--gap);
	}

	.meta {
		font-size: 0.8em;
		opacity: 0.75;
		display: flex;
		align-items: center;
		flex-wrap: wrap;
		gap: 4px;
	}

	.comment {
		border-bottom: solid 1px var(--sep-color);
		padding-bottom: var(--gap);
	}

	.delete-comment-btn {
		background: none;
		border: none;
		padding: 0;
		margin: 0;
		color: inherit;
		cursor: pointer;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		transition:
			color 0.2s ease,
			transform 0.1s ease;
		opacity: 0.8;
		width: 14px;
		height: 14px;
	}

	.delete-comment-btn:hover:not(:disabled) {
		color: #e53e3e;
		transform: scale(1.15);
		opacity: 1;
	}

	.delete-comment-btn:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}
</style>
