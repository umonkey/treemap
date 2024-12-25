<script lang="ts">
	import { formatSpecies } from '$lib/utils/trees';
	import { get } from 'svelte/store';
	import { getTree } from '$lib/stores/treeStore';
	import { routes } from '$lib/routes';

	import LazyTreeThumbnail from '$lib/components/LazyTreeThumbnail.svelte';

	const { id } = $props();
	const tree = get(getTree)(id);
</script>

<div class="tree">
	<div class="thumbnail">
		<a href={routes.treeDetails(tree.id)}>
			<LazyTreeThumbnail {tree} />
		</a>
	</div>

	<div class="species">
		<strong>{formatSpecies(tree.species)}</strong>{#if tree.address}
			- {tree.address}{/if}
	</div>
</div>

<style>
	.tree {
		display: flex;
		flex-direction: row;
		gap: var(--gap);
		height: 50px;
		padding: var(--gap) 0;
		line-height: 50px;
	}

	.thumbnail {
		flex-basis: 50px;

		width: 100%;
		height: 100%;
		object-fit: cover;
		object-position: center;
	}

	strong {
		font-size: 20px;
	}
</style>
