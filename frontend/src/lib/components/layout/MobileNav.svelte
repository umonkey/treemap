<script lang="ts">
	import { BellIcon, HomeIcon, MapIcon, SearchIcon, UserIcon, SpinnerIcon } from '$lib/icons';
	import { routes } from '$lib/routes';
	import { authStore, isAuthenticated } from '$lib/stores/authStore';
	import { uploadStore } from '$lib/stores/upload';
	import UserPic from '$lib/components/layout/UserPic.svelte';
</script>

<nav class="mobile">
	<a href="/"><div><HomeIcon /></div></a>
	<a href="/search"><div><SearchIcon /></div></a>
	<a href={routes.map()}><div><MapIcon /></div></a>
	<a href={routes.treeUpdates()}><div><BellIcon /></div></a>
	<a href="/profile"
		><div>
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
		</div></a
	>
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
