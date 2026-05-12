<script lang="ts">
	import { untrack } from 'svelte';
	import FALLBACK from '$lib/assets/tree.jpg';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import InfiniteScroll from '$lib/components/layout/InfiniteScroll.svelte';
	import { locale } from '$lib/locale';
	import LazyImage from '$lib/ui/lazy-image/LazyImage.svelte';
	import SignInButton from '$lib/ui/sign-in-button/SignInButton.svelte';
	import { pageState } from './page.svelte';

	$effect(() => {
		untrack(() => pageState.load());

		return () => {
			untrack(() => pageState.reset());
		};
	});
</script>

<Dialog title={locale.savedTitle()}>
	{#if pageState.loading && pageState.tiles.length === 0}
		<p>Loading saved trees...</p>
	{:else if pageState.statusCode === 401}
		<div class="signed-out">
			<p>{locale.profileSignInPrompt()}</p>
			<SignInButton />
		</div>
	{:else if pageState.error}
		<p>Error loading saved trees.</p>
	{:else if pageState.tiles.length === 0}
		<p>You haven't saved any trees yet.</p>
	{:else}
		<div class="tiles">
			<InfiniteScroll
				onLoadMore={pageState.handleLoadMore}
				enabled={!pageState.loading && pageState.hasMore}
			>
				{#each pageState.tiles as tile (tile.id)}
					<div class="tile">
						<a href={tile.link}>
							<LazyImage src={tile.image ?? FALLBACK} fallback={FALLBACK} alt={tile.species} />

							<div class="meta">
								<div>{tile.updated_at} &middot; {tile.species}</div>
								<div>{tile.address}</div>
							</div>
						</a>
					</div>
				{/each}
			</InfiniteScroll>
		</div>
	{/if}
</Dialog>

<style>
	.signed-out {
		text-align: center;
		padding: 2rem;
	}

	.tiles {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
		grid-gap: var(--gap);

		.tile {
			aspect-ratio: 1/1;

			:global(img) {
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

	@media screen and (max-width: 600px) {
		.tiles {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
