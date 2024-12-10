<script>
	import { isAuthenticated, authState } from '$lib/stores/auth';
	import SignIn from '$lib/components/auth/SignIn.svelte';
	import SignOut from '$lib/components/auth/SignOut.svelte';
	import BackIcon from '$lib/icons/BackIcon.svelte';

	const onBack = () => {
		history.back();
	};
</script>

{#if $isAuthenticated}
	<img class="header" src="/header.jpg" alt="header background" />

	<button class="back" on:click={onBack}><BackIcon width="20px" height="20px" /></button>

	<div class="container signedIn">
		<img class="userpic" src={$authState.picture} alt="userpic" />

		<h1>{$authState.name}</h1>
		<div class="stats"><em>75</em> trees, <em>14</em> edits, <em>295</em> photos</div>

		<div class="actions">
			<SignOut />
		</div>
	</div>
{:else}
	<div class="container signedOut">
		<h1>Profile</h1>
		<p>You need to sign in to access your profile.</p>
		<SignIn />
	</div>
{/if}

<style>
	img.header {
		width: 100%;
		height: 40vw;
		object-position: center;
		object-fit: cover;
		z-index: 4;
	}

	.container {
		padding: 0 var(--gap);

		&.signedIn {
			margin-top: -45px;
		}
	}

	img.userpic {
		width: 90px;
		height: 90px;
		border-radius: 50%;
		display: block;
		box-shadow: 2px 2px 5px 0px rgba(0, 0, 0, 1);
		z-index: 5;
	}

	h1 {
		font-weight: 800;
		font-size: 24px;
		margin: 10px 0 5px;
	}

	em {
		font-style: normal;
		font-weight: 800;
	}

	.actions {
		margin-top: var(--gap);
	}

	button.back {
		width: 30px;
		height: 30px;
		border: none;
		position: absolute;
		top: var(--gap);
		left: var(--gap);
		border-radius: 50%;
		background-color: var(--form-background);
		cursor: pointer;
	}
</style>
