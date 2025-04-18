<script lang="ts">
	import { apiClient } from '$lib/api';
	import { routes } from '$lib/routes';
	import type { ITree } from '$lib/types';

	const { tree } = $props<{
		tree: ITree;
	}>();
</script>

<div class="gallery">
	{#await apiClient.getTree(tree.id) then res}
		{#if res.status === 200 && res.data && res.data.files.length > 0}
			<div class="images">
				{#each res.data.files as file}
					<div>
						<a href={routes.treeDetails(tree.id)}>
							<img src={routes.file(file.small_id)} alt="See how good is this tree." />
						</a>
					</div>
				{/each}
			</div>
		{:else if res.status === 200 && res.data && res.data.files.length === 0}
			<div class="images">
				<a href={routes.treeDetails(tree.id)}>
					<img src="/tree.jpg" alt="See how good is this tree." />
				</a>
			</div>
		{:else}
			Failed to load photos.
		{/if}
	{/await}
</div>

<style>
	.gallery {
		height: 50px;
		line-height: 50px;
		margin-top: var(--gap);
	}

	.images {
		display: flex;
		flex-direction: row;
		gap: var(--gap);

		scroll-snap-type: x mandatory;
		scrollbar-width: none;

		a {
			display: block;

			scroll-snap-align: start;
			scroll-snap-stop: always;

			img {
				display: block;
				width: 50px;
				height: 50px;
				object-position: center;
				object-fit: cover;
				overflow: hidden;
				color: transparent; /* hide alt */
			}
		}
	}
</style>
