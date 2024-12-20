<script lang="ts">
	import { routes } from '$lib/routes';

	import BellIcon from '$lib/icons/BellIcon.svelte';
	import HomeIcon from '$lib/icons/HomeIcon.svelte';
	import MapIcon from '$lib/icons/MapIcon.svelte';
	import PlusIcon from '$lib/icons/PlusIcon.svelte';
	import SearchIcon from '$lib/icons/SearchIcon.svelte';
	import UserIcon from '$lib/icons/UserIcon.svelte';
</script>

<aside class="left">
	<ul>
		<li>
			<a href="/">
				<span class="icon"><HomeIcon /></span>
				<span>Home</span>
			</a>
		</li>
		<li>
			<a href="/search">
				<span class="icon"><SearchIcon /></span>
				<span>Search</span>
			</a>
		</li>
		<li>
			<a href="/map">
				<span class="icon"><MapIcon /></span>
				<span>Explore Map</span>
			</a>
		</li>
		<li>
			<a href={routes.newTrees()}>
				<span class="icon"><BellIcon /></span>
				<span>Recent updates</span>
			</a>
		</li>
		<li>
			<a href="/profile">
				<span class="icon"><UserIcon /></span>
				<span>Profile</span>
			</a>
		</li>
		<li>
			<a href="/add">
				<span class="icon"><PlusIcon /></span>
				<span>Add a tree</span>
			</a>
		</li>
	</ul>
</aside>

<style>
	aside {
		flex-basis: 300px;
		flex-shrink: 0;
		flex-grow: 0;

		padding: calc(2 * var(--gap));
		box-sizing: border-box;
		border-right: 1px solid var(--sep-color);

		text-align: left;
		font-size: 18px;

		color: var(--text-color);
	}

	ul {
		list-style-type: none;
	}

	a {
		display: flex;
		flex-direction: row;
		gap: var(--gap);
		line-height: 24px;
		color: inherit;
		text-decoration: none;
		padding: 10px 0;
		margin-bottom: 10px;
	}

	.icon {
		flex-basis: 30px;
		flex-shrink: 0;
		flex-grow: 0;
		height: 24px;
	}

	@media (max-width: 400px) {
		aside {
			display: none;
		}
	}
</style>
