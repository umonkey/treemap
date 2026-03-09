<script lang="ts">
	import { NarrowPage } from '$lib/ui';
	import Tabs from '$lib/components/updates/Tabs.svelte';
	import InfiniteScroll from '$lib/components/layout/InfiniteScroll.svelte';
	import { locale } from '$lib/locale';
	import { hooks } from './hooks';
	import FALLBACK from '$lib/assets/tree.jpg';

	const { loading, error, tiles, handleLoadMore } = hooks();
</script>

<NarrowPage title={locale.updatesNewTitle()} nopadding>
	<div class="spacer">
		<Tabs active="trees" />

		{#if $loading}
			<p>Loading trees...</p>
		{:else if $error}
			<p>Error loading trees.</p>
		{:else}
			<div class="tiles">
				<InfiniteScroll onLoadMore={handleLoadMore}>
					{#each $tiles as tile}
						<div class="tile">
							<a href={tile.link}>
								<img src={tile.image ?? FALLBACK} alt={tile.species} />

								<div class="meta">
									<div>{tile.species}</div>
									<div>{tile.address}</div>
									<div>{tile.updated_at} &middot; {tile.user_name}</div>
								</div>
							</a>
						</div>
					{/each}
				</InfiniteScroll>
			</div>
		{/if}
	</div>
</NarrowPage>

<style>
	.tiles {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		grid-gap: var(--gap);

		.tile {
			aspect-ratio: 1/1;

			img {
				width: 100%;
				aspect-ratio: 1/1;
				object-position: center;
				object-fit: cover;
			}
		}

		a {
			display: block;
			line-height: 0;

			text-decoration: none;
			color: inherit;
			position: relative;
		}

		.meta {
			position: absolute;
			bottom: 0;
			padding-top: 10px;
			background: linear-gradient(
				to bottom,
				rgba(0, 0, 0, 0) 0%,
				rgba(0, 0, 0, 0.75) 10%,
				rgba(0, 0, 0, 1) 100%
			);
			font-size: 0.8rem;
			width: 100%;

			div {
				padding: 0 4px;
				vertical-align: bottom;
				height: 20px;
				line-height: 20px;

				white-space: nowrap;
				overflow: hidden;
				text-overflow: ellipsis;
			}
		}
	}

	.spacer {
		display: flex;
		flex-direction: column;
		gap: var(--gap);
	}

	@media screen and (max-width: 600px) {
		.tiles {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
