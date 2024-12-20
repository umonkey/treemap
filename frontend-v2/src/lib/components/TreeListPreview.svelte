<script lang="ts">
	import { routes } from '$lib/routes';
	import { getTree } from '$lib/stores/treeStore';
	import { get } from 'svelte/store';

	const { id } = $props();
	const tree = get(getTree)(id);

	const thumbnail = (id: string | undefined): string => {
		if (id) {
			return routes.file(id);
		}

		return '/icons/tree.png';
	};
</script>

<div class="tree">
	<img src={thumbnail(tree.thumbnail_id)} alt={tree.species} />
	<div class="species">
		<strong>{tree.species}</strong>{#if tree.address}
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

	img {
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
