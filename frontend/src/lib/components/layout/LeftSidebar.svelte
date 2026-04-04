<script lang="ts">
	import Logo from '$lib/assets/trees-of-yerevan.svelte';
	import UserPic from '$lib/components/layout/UserPic.svelte';
	import { BellIcon, CloseIcon, HomeIcon, SearchIcon, SpinnerIcon } from '$lib/icons';
	import ChartIcon from '$lib/icons/ChartIcon.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { authStore } from '$lib/stores/authStore';
	import { isSidebarVisible, mobileSidebarStore } from '$lib/stores/mobileSidebarStore';
	import { uploadStore } from '$lib/stores/upload';
	import { onMount } from 'svelte';
	import { componentState } from './LeftSidebar.svelte.ts';

	onMount(() => {
		componentState.fetchStats();
	});

	const onClose = () => {
		mobileSidebarStore.set(false);
	};
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div class="overlay" class:visible={!!$isSidebarVisible} onclick={onClose}>
	<div class="canvas" onclick={(e) => e.stopPropagation()}>
		<div class="mobile-header">
			<button class="close-btn" onclick={onClose}>
				<CloseIcon />
			</button>
		</div>

		<ul>
			<li>
				<a href="/" onclick={onClose}>
					<span class="icon"><HomeIcon /></span>
					<span>{locale.sideHome()}</span>
				</a>
			</li>
			<li>
				<a href="/search" onclick={onClose}>
					<span class="icon"><SearchIcon /></span>
					<span>{locale.sideSearch()}</span>
				</a>
			</li>
			<li>
				<a href="/stats" onclick={onClose}>
					<span class="icon"><ChartIcon /></span>
					<span>{locale.sideReports()}</span>
				</a>
			</li>
			<li>
				<a href={routes.treeUpdates()} onclick={onClose}>
					<span class="icon"><BellIcon /></span>
					<span>{locale.sideUpdates()}</span>
				</a>
			</li>
			<li>
				<a href={routes.profile()} onclick={onClose}>
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

		<div class="stats">
			{#if componentState.stats}
				{locale.sideStatsMessage(componentState.stats.count)}
			{/if}
		</div>

		<div class="bottom">
			<Logo />
		</div>

		<div class="bottom links">
			<a
				href="https://github.com/KanachYerevan/kb/wiki/Mobile-Application"
				target="_blank"
				onclick={onClose}>{locale.sideAbout()}</a
			>
			&middot;
			<a href="https://github.com/umonkey/treemap/issues" target="_blank" onclick={onClose}
				>{locale.sideBugs()}</a
			>
		</div>
	</div>
</div>

<style>
	.canvas {
		height: 100vh;
		width: 300px;
		position: fixed;
		top: 0;
		left: 0;
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

		.mobile-header {
			display: none;
		}

		ul {
			margin: 0;
			padding: 0 var(--gap);
			list-style-type: none;
			flex: 0 0;

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

		.stats {
			flex: 1 1;
			border-top: 1px solid rgba(128, 128, 128, 0.2);
			padding: 1rem 0 0;

			font-size: 1rem;
			opacity: 0.8;
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

	/** Hide by default on small screens. **/
	@media screen and (max-width: 1023px) {
		.overlay {
			animation: fadeIn 0.1s ease-in-out;

			display: none;
			position: fixed;
			top: 0;
			left: 0;
			width: 100vw;
			height: 100dvh;
			background-color: rgba(0, 0, 0, 0.5);
			backdrop-filter: blur(2px);

			&.visible {
				display: block;
			}
		}

		.canvas {
			width: 100vw;
			height: 100dvh;
			border-right: none;

			.mobile-header {
				display: flex;
				justify-content: flex-end;
				padding-bottom: var(--gap);

				.close-btn {
					background: none;
					border: none;
					color: inherit;
					width: 32px;
					height: 32px;
					padding: 0;
					cursor: pointer;
				}
			}
		}
	}

	/** Must be belog the preview on desktop, above on mobile. **/
	.overlay {
		z-index: var(--z-learn-result);
	}

	@media screen and (min-width: 1024px) {
		.overlay {
			z-index: 1;
		}
	}
</style>
