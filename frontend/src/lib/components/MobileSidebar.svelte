<script lang="ts">
import { locale } from "$lib/locale";
import { routes } from "$lib/routes";
import {
	isSidebarVisible,
	mobileSidebarStore,
} from "$lib/stores/mobileSidebarStore";

import BellIcon from "$lib/icons/BellIcon.svelte";
import ChartIcon from "$lib/icons/ChartIcon.svelte";
import HomeIcon from "$lib/icons/HomeIcon.svelte";
import LearnIcon from "$lib/icons/LearnIcon.svelte";
import MapIcon from "$lib/icons/MapIcon.svelte";
import PlusIcon from "$lib/icons/PlusIcon.svelte";
import SearchIcon from "$lib/icons/SearchIcon.svelte";
import UserIcon from "$lib/icons/UserIcon.svelte";

const onClick = () => {
	mobileSidebarStore.update(() => false);
};
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
	class="overlay sidebar"
	class:visible={$isSidebarVisible}
	class:hidden={!$isSidebarVisible}
	onclick={onClick}
>
	<aside>
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
				<a href={routes.learn()}>
					<span class="icon"><LearnIcon /></span>
					<span>{locale.sideLearn()}</span>
				</a>
			</li>
			<li>
				<a href="/add">
					<span class="icon"><PlusIcon /></span>
					<span>{locale.sideAdd()}</span>
				</a>
			</li>
		</ul>
		<div class="links">
			<a href="https://github.com/KanachYerevan/kb/wiki/Mobile-Application" target="_blank"
				>{locale.sideAbout()}</a
			>
		</div>
	</aside>
</div>

<style>
	.sidebar.hidden {
		display: none;
	}

	.overlay {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100dvh;
		z-index: var(--z-mobile-sidebar);

		background-color: rgba(0, 0, 0, 0.5);
		backdrop-filter: blur(2px);
	}

	aside {
		position: fixed;
		top: 0;
		left: 0;
		height: 100%;
		z-index: var(--z-mobile-sidebar);

		padding: calc(2 * var(--gap));
		box-sizing: border-box;
		border-right: 1px solid var(--sep-color);

		text-align: left;
		font-size: 16px;

		background-color: var(--form-background);
		color: var(--text-color);
	}

	.links {
		font-size: 14px;
		color: var(--link-color);
		position: absolute;
		bottom: 0;
	}

	ul {
		list-style-type: none;
		margin: 0;
		padding: 0;
		white-space: nowrap;
		min-width: 200px;
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
		flex-basis: 24px;
		flex-shrink: 0;
		flex-grow: 0;
		height: 24px;
	}

	@media (min-width: 480px) {
		aside {
			display: none;
		}
	}
</style>
