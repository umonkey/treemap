<script>
import BellIcon from "$lib/icons/BellIcon.svelte";
import HomeIcon from "$lib/icons/HomeIcon.svelte";
import MapIcon from "$lib/icons/MapIcon.svelte";
import SearchIcon from "$lib/icons/SearchIcon.svelte";
import UserIcon from "$lib/icons/UserIcon.svelte";
import { routes } from "$lib/routes";
import { authStore, isAuthenticated } from "$lib/stores/authStore";
</script>

<nav class="mobile">
	<a href="/"><div><HomeIcon /></div></a>
	<a href="/search"><div><SearchIcon /></div></a>
	<a href="/map"><div><MapIcon /></div></a>
	<a href={routes.newTrees()}><div><BellIcon /></div></a>
	<a href="/profile"
		><div>
			{#if $isAuthenticated && $authStore?.picture}
				<img src={$authStore.picture} alt="userpic" />
			{:else}
				<UserIcon />
			{/if}
		</div></a
	>
</nav>

<style>
	nav {
		position: fixed;
		bottom: 0;
		width: 100%;
		height: 50px;
		line-height: 50px;
		text-align: center;

		display: flex;
		flex-direction: row;

		background-color: var(--background-color);
		border-top: 1px solid var(--sep-color);

		z-index: var(--z-mobile-nav);
	}

	a {
		display: block;
		align-content: center;
		color: inherit;

		line-height: 24px;

		flex-basis: 20%;
		flex-shrink: 0;
		flex-grow: 0;

		div {
			margin: 0 auto;
			width: 24px;
			height: 24px;
		}
	}

	img {
		width: 24px;
		height: 24px;
		border-radius: 50%;
	}

	@media (min-width: 480px) {
		nav {
			display: none;
		}
	}
</style>
