<script lang="ts">
	import { getUser } from '$lib/stores/userStore';
	import type { IComment } from '$lib/types';
	import { formatDate } from '$lib/utils/strings';

	const { comment }: { comment: IComment } = $props();

	const user = $derived($getUser(comment.added_by));
	const date = $derived(formatDate(comment.added_at));
</script>

<div class="comment">
	<blockquote>{comment.message}</blockquote>
	<div class="meta">
		{date} · {#if user}{user.name}{:else}unknown user{/if}
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
	}

	.comment {
		border-bottom: solid 1px var(--sep-color);
		padding-bottom: var(--gap);
	}
</style>
