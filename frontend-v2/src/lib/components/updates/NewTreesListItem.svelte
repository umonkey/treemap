<script lang="ts">
	import { formatDate } from '$lib/utils/strings';
	import { get } from 'svelte/store';
	import { getUser } from '$lib/stores/userStore';
	import { routes } from '$lib/routes';

	import LazyTreeThumbnail from '$lib/components/LazyTreeThumbnail.svelte';

	const { tree } = $props();

	const user = get(getUser)(tree.added_by);
	const date = formatDate(tree.added_at);
</script>

<div class="tree">
	<a href={routes.treeDetails(tree.id)} class="thumbnail">
		<LazyTreeThumbnail {tree} />
	</a>

	<div class="details">
		<div class="species">
			<a href={routes.treeDetails(tree.id)}
				>{#if tree.species}{tree.species}{:else}Unknown species{/if}</a
			>
		</div>
		<div class="address">
			{#if tree.address}{tree.address}{:else}Unknown address{/if}
		</div>
		<div class="added">{date} &middot; {user.name}</div>
	</div>
</div>

<style>
	.tree {
		display: flex;
		flex-direction: row;
		gap: var(--gap);
		margin-bottom: var(--gap);
	}

	.thumbnail {
		flex-basis: 75px;
		flex-shrink: 0;
		flex-grow: 0;
	}

	.details {
		line-height: 150%;
		flex-grow: 1;
	}
</style>
