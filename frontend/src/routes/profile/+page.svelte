<script lang="ts">
import SignIn from "$lib/components/auth/SignIn.svelte";
import SignOut from "$lib/components/auth/SignOut.svelte";
import Header from "$lib/components/tree/Header.svelte";
import { loadMe } from "$lib/hooks";
import { locale } from "$lib/locale";

const { loading, error, data, statusCode, reload } = loadMe();

$effect(() => {
	reload();
});
</script>

<svelte:head>
	<title>Profile</title>
</svelte:head>

<Header title="Profile" />

{#if $loading}
	<!-- loading --->
{:else if $statusCode === 401}
	<div class="container signedOut">
		<p>{locale.profileSignInPrompt()}</p>
		<SignIn />
	</div>
{:else if $error}
	<p>{$error.description}</p>
{:else if $data}
	<img class="header" src="/header.jpg" alt="header background" />

	<div class="container signedIn">
		<img class="userpic" src={$data.picture} alt="userpic" />

		<h1>{$data.name}</h1>
		<div class="stats">
			{locale.profileTrees($data.trees_count)}, {locale.profileUpdates($data.updates_count)}, {locale.profilePhotos(
				$data.files_count
			)}.
		</div>

		<div class="actions">
			<SignOut />
		</div>
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
