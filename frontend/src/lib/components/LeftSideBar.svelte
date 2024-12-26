<script lang="ts">
	import { routes } from '$lib/routes';
	import { locale } from '$lib/locale';

	import ChartIcon from '$lib/icons/ChartIcon.svelte';
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
				<span>{locale.sideHome()}</span>
			</a>
		</li>
		<li>
			<a href="/search">
				<span class="icon"><SearchIcon /></span>
				<span>{locale.sideSearch()}</span>
			</a>
		</li>
		<li>
			<a href="/map">
				<span class="icon"><MapIcon /></span>
				<span>{locale.sideExplore()}</span>
			</a>
		</li>
		<li>
			<a href={routes.newTrees()}>
				<span class="icon"><BellIcon /></span>
				<span>{locale.sideUpdates()}</span>
			</a>
		</li>
		<li>
			<a href={routes.stats()}>
				<span class="icon"><ChartIcon /></span>
				<span>{locale.sideReports()}</span>
			</a>
		</li>
		<li>
			<a href="/profile">
				<span class="icon"><UserIcon /></span>
				<span>{locale.sideProfile()}</span>
			</a>
		</li>
		<li>
			<a href="/add">
				<span class="icon"><PlusIcon /></span>
				<span>{locale.sideAdd()}</span>
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
