<script lang="ts">
	import Logo from '$lib/assets/trees-of-yerevan.svelte';
	import UserPic from '$lib/components/layout/UserPic.svelte';
	import BellIcon from '$lib/icons/BellIcon.svelte';
	import ChartIcon from '$lib/icons/ChartIcon.svelte';
	import ChatIcon from '$lib/icons/ChatIcon.svelte';
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import HomeIcon from '$lib/icons/HomeIcon.svelte';
	import InstallIcon from '$lib/icons/InstallIcon.svelte';
	import SaveIcon from '$lib/icons/SaveIcon.svelte';
	import SearchIcon from '$lib/icons/SearchIcon.svelte';
	import SpinnerIcon from '$lib/icons/SpinnerIcon.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { authStore } from '$lib/stores/authStore';
	import { isSidebarVisible } from '$lib/stores/mobileSidebarStore';
	import { isInstallable } from '$lib/stores/pwaStore';
	import { uploadStore } from '$lib/stores/upload';
	import { componentState } from './LeftSidebar.svelte.ts';
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
	class="overlay"
	class:visible={!!$isSidebarVisible}
	onclick={componentState.handleOverlayClick}
>
	<div class="canvas">
		<div class="mobile-header">
			<button class="close-btn" onclick={componentState.close}>
				<CloseIcon />
			</button>
		</div>

		<ul>
			<li>
				<a href={routes.home()} onclick={componentState.close}>
					<span class="icon"><HomeIcon /></span>
					<span>{locale.sideHome()}</span>
				</a>
			</li>
			<li>
				<a href={routes.search()} onclick={componentState.close}>
					<span class="icon"><SearchIcon /></span>
					<span>{locale.sideSearch()}</span>
				</a>
			</li>
			<li>
				<a href={routes.stats()} onclick={componentState.close}>
					<span class="icon"><ChartIcon /></span>
					<span>{locale.sideReports()}</span>
				</a>
			</li>
			<li>
				<a href={routes.treeUpdates()} onclick={componentState.close}>
					<span class="icon"><BellIcon /></span>
					<span>{locale.sideUpdates()}</span>
				</a>
			</li>
			<li>
				<a href={routes.treeSaved()} onclick={componentState.close}>
					<span class="icon"><SaveIcon /></span>
					<span>{locale.sideSaved()}</span>
				</a>
			</li>
			{#if $isInstallable}
				<li>
					<button class="menu-item" onclick={componentState.install}>
						<span class="icon"><InstallIcon /></span>
						<span>{locale.sideInstallApp()}</span>
					</button>
				</li>
			{/if}
			<li>
				<a
					href="https://t.me/kanach_yerevan_bot"
					target="_blank"
					rel="noopener noreferrer"
					onclick={componentState.close}
				>
					<span class="icon"><ChatIcon /></span>
					<span>{locale.sideReportDamage()}</span>
				</a>
			</li>
			<li>
				<a href={routes.profile()} onclick={componentState.close}>
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

		<div class="sep"></div>

		<div class="bottom">
			<div class="logo">
				<Logo />
			</div>

			<div class="links">
				<a
					href="https://github.com/KanachYerevan/kb/wiki/Mobile-Application"
					target="_blank"
					onclick={componentState.close}>{locale.sideAbout()}</a
				>
				&middot;
				<a
					href="https://github.com/umonkey/treemap/issues"
					target="_blank"
					onclick={componentState.close}>{locale.sideBugs()}</a
				>
				&middot;
				<a href={routes.privacy()} onclick={componentState.close}>{locale.sidePrivacy()}</a>
			</div>
		</div>
	</div>
</div>

<style>
	.canvas {
		height: 100dvh;
		width: 300px;
		position: fixed;
		top: 0;
		left: 0;
		box-sizing: border-box;

		background-color: var(--map-menu-background);

		display: flex;
		flex-direction: column;
		gap: calc(var(--gap) * 2);

		padding: calc(2 * var(--gap));
		padding-bottom: calc(2 * var(--gap));
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

			a,
			button.menu-item {
				display: flex;
				flex-direction: row;
				gap: var(--gap);
				line-height: 24px;
				color: inherit;
				text-decoration: none;
				padding: 10px 0;
				margin-bottom: 10px;
				background: none;
				border: none;
				width: 100%;
				text-align: left;
				font: inherit;
				cursor: pointer;
			}

			a {
				cursor: pointer;
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
			text-align: center;

			display: flex;
			flex-direction: column;
			gap: 1rem;
			align-items: center;
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

	.sep {
		flex: 1 1;
	}

	/** Hide by default on small screens. **/
	@media screen and (max-width: 1023px) {
		.overlay {
			animation: fadeIn 0.2s ease-in-out;
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
			height: auto;
			top: auto;
			bottom: 0;
			border-right: none;
			border-radius: 16px 16px 0 0;
			padding: var(--gap);
			padding-bottom: calc(2 * var(--gap));
			animation: slideUp 0.2s ease-out;

			.mobile-header {
				position: absolute;
				top: var(--gap);
				right: var(--gap);
				z-index: 1;
				display: block;

				.close-btn {
					background-color: transparent;
					border: none;
					color: light-dark(black, white);
					width: 30px;
					height: 30px;
					padding: 0 4px;
					cursor: pointer;
					opacity: 0.5;
				}
			}

			.logo {
				display: none;
			}

			ul {
				padding-bottom: var(--gap);
			}
		}

		.sep {
			display: none;
		}
	}

	@keyframes slideUp {
		from {
			transform: translateY(100%);
		}
		to {
			transform: translateY(0);
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
