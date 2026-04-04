<script lang="ts">
	import UserPic from '$lib/components/layout/UserPic.svelte';
	import BellIcon from '$lib/icons/BellIcon.svelte';
	import HomeIcon from '$lib/icons/HomeIcon.svelte';
	import SearchIcon from '$lib/icons/SearchIcon.svelte';
	import SpinnerIcon from '$lib/icons/SpinnerIcon.svelte';
	import UserIcon from '$lib/icons/UserIcon.svelte';
	import { routes } from '$lib/routes';
	import { authStore, isAuthenticated } from '$lib/stores/authStore';
	import { mobileSidebarStore } from '$lib/stores/mobileSidebarStore';
	import { uploadStore } from '$lib/stores/upload';

	const toggleSidebar = () => {
		mobileSidebarStore.set(true);
	};
</script>

<nav class="mobile">
	<a href="/"><div><HomeIcon /></div></a>
	<a href="/search"><div><SearchIcon /></div></a>
	<a href={routes.treeUpdates()}><div><BellIcon /></div></a>
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="nav-item" onclick={toggleSidebar}>
		<div>
			{#if $uploadStore.uploading}
				<SpinnerIcon />
			{:else if $isAuthenticated}
				<UserPic src={$authStore?.picture} alt="userpic" class="user-pic-nav" />
			{:else}
				<UserIcon />
			{/if}
			{#if $uploadStore.pending > 0}
				<span class="badge">{$uploadStore.pending}</span>
			{/if}
		</div>
	</div>
</nav>

<style>
	nav {
		position: fixed;
		bottom: 0;
		width: 100%;
		height: var(--bottom-nav-height);
		line-height: var(--bottom-nav-height);
		text-align: center;

		display: flex;
		flex-direction: row;

		background-color: var(--background-color);
		border-top: 1px solid var(--sep-color);

		z-index: var(--z-mobile-nav);
	}

	a,
	.nav-item {
		display: block;
		align-content: center;
		color: inherit;
		cursor: pointer;

		line-height: 24px;

		flex-basis: 25%;
		flex-shrink: 0;
		flex-grow: 0;

		div {
			margin: 0 auto;
			width: 24px;
			height: 24px;
			position: relative;
		}
	}

	:global(.user-pic-nav) {
		width: 24px;
		height: 24px;
		border-radius: 50%;
	}

	@media (min-width: 1024px) {
		nav {
			display: none;
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
