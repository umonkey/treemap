<script lang="ts">
	import { isAuthenticated, authStore } from '$lib/stores/authStore';
	import { locale } from '$lib/locale';
	import SignIn from '$lib/components/auth/SignIn.svelte';
	import SignOut from '$lib/components/auth/SignOut.svelte';
	import Header from '$lib/components/tree/Header.svelte';

	const { data } = $props();
	const { profile } = data;
</script>

<svelte:head>
	<title>Profile</title>
</svelte:head>

<Header title="Profile" />

{#if $isAuthenticated}
	<img class="header" src="/header.jpg" alt="header background" />

	<div class="container signedIn">
		<img class="userpic" src={$authStore?.picture} alt="userpic" />

		<h1>{$authStore?.name ?? 'Unknown user'}</h1>
		<div class="stats">
			{locale.profileTrees(profile.trees_count)}, {locale.profileUpdates(profile.updates_count)}, {locale.profilePhotos(
				profile.files_count
			)}.
		</div>

		<div class="actions">
			<SignOut />
		</div>
	</div>
{:else}
	<div class="container signedOut">
		<p>{locale.profileSignInPrompt()}</p>
		<SignIn />
	</div>
{/if}

<style>
	img.header {
		display: block;
		position: relative;
		width: 100%;
		object-position: center;
		object-fit: cover;
		z-index: 1;
	}

	.container {
		padding: 0 var(--gap);
		position: relative;
		z-index: 2;

		&.signedIn {
			margin-top: -45px;
		}
	}

	img.userpic {
		width: 90px;
		height: 90px;
		border-radius: 50%;
		background-color: #000;
		color: transparent; /* hide alt */
		display: block;
		box-shadow: 2px 2px 5px 0px rgba(0, 0, 0, 1);
		z-index: 2;
	}

	h1 {
		font-weight: 800;
		font-size: 24px;
		margin: 10px 0 5px;
	}

	.actions {
		margin-top: var(--gap);
	}

	@media (max-width: 480px) {
		img.header {
			height: 40vw;
		}
	}

	@media (min-width: 481px) {
		img.header {
			height: 200px;
		}
	}
</style>
