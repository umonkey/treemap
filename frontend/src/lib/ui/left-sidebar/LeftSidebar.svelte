<script lang="ts">
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { BellIcon, HomeIcon, MapIcon, SearchIcon, SpinnerIcon } from '$lib/icons';
	import { mapLastTree } from '$lib/stores/mapStore';
	import { searchStore } from '$lib/stores';
	import { uploadStore } from '$lib/stores/upload';
	import { authStore } from '$lib/stores/authStore';
	import Logo from '$lib/assets/trees-of-yerevan.svelte';
	import UserPic from '$lib/components/nav/UserPic.svelte';
</script>

<aside class="left">
	<div class="canvas">
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
			{#if $mapLastTree}
				<li>
					<a href={routes.mapPreview($mapLastTree, $searchStore)}>
						<span class="icon"><MapIcon /></span>
						<span>{locale.sideExplore()}</span>
					</a>
				</li>
			{:else if $searchStore}
				<li>
					<a href={routes.searchQuery($searchStore)}>
						<span class="icon"><MapIcon /></span>
						<span>{locale.sideExplore()}</span>
					</a>
				</li>
			{:else}
				<li>
					<a href={routes.map()}>
						<span class="icon"><MapIcon /></span>
						<span>{locale.sideExplore()}</span>
					</a>
				</li>
			{/if}
			<li>
				<a href={routes.newTrees()}>
					<span class="icon"><BellIcon /></span>
					<span>{locale.sideUpdates()}</span>
				</a>
			</li>
			<li>
				<a href={routes.profile()}>
					<span class="icon">
						{#if $uploadStore.uploading}
							<SpinnerIcon />
						{:else}
							<UserPic src={$authStore?.picture} alt="userpic" class="user-pic-sidebar" />
						{/if}
						{#if $uploadStore.pending > 0}
							<span class="badge">{$uploadStore.pending}</span>
						{/if}
					</span>
					<span>{locale.sideProfile()}</span>
				</a>
			</li>
		</ul>

		<div class="bottom">
			<Logo />
		</div>

		<div class="bottom links">
			<a href="https://github.com/KanachYerevan/kb/wiki/Mobile-Application" target="_blank"
				>{locale.sideAbout()}</a
			>
			&middot;
			<a href="https://github.com/umonkey/treemap/issues" target="_blank">{locale.sideBugs()}</a>
		</div>
	</div>
</aside>

<style>
	.canvas {
		height: 100vh;
		width: 300px;
		position: fixed;
		box-sizing: border-box;

		background-color: var(--form-background);

		display: flex;
		flex-direction: column;
		gap: calc(var(--gap) * 2);

		padding: calc(2 * var(--gap));
		border-right: 1px solid var(--sep-color);

		text-align: left;
		font-size: 18px;

		color: var(--text-color);

		ul {
			margin: 0;
			padding: 0 var(--gap);
			list-style-type: none;
			flex-grow: 1;

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
				width: 24px;
				position: relative;
			}

			:global(.user-pic-sidebar) {
				width: 24px;
				height: 24px;
				border-radius: 50%;
				margin-left: 4px;
			}
		}

		.bottom {
			flex-grow: 0;
			flex-shrink: 0;
			opacity: 0.5;
		}

		.links {
			font-size: 14px;
			text-align: center;

			a {
				color: inherit;
				text-decoration: underline;
			}
		}
	}

	.badge {
		position: absolute;
		top: 0;
		left: 100%;
		transform: translate(-50%, -50%);

		font-size: 0.8rem;
		background-color: #080;
		padding: 0 0.5rem;
		border-radius: 5px;
	}
</style>
