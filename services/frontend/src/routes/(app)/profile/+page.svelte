<script lang="ts">
	import { untrack } from 'svelte';
	import UserHeatMap from '$lib/components/UserHeatMap/index.svelte';
	import { locale } from '$lib/locale';
	import SignInButton from '$lib/ui/sign-in-button/SignInButton.svelte';
	import ProfileHeader from './ProfileHeader.svelte';
	import { pageState } from './page.svelte';

	$effect(() => {
		untrack(() => pageState.reload());
	});
</script>

<svelte:head>
	<title>{locale.sideProfile()} — {locale.appTitle()}</title>
</svelte:head>

{#if pageState.loading}
	...
{:else if pageState.statusCode === 401}
	<div class="container signedOut">
		<p>{locale.profileSignInPrompt()}</p>
		<SignInButton />
	</div>
{:else if pageState.error}
	<p>{pageState.error.description}</p>
{:else if pageState.data}
	<ProfileHeader
		name={pageState.data.user.name}
		userpic={pageState.data.user.picture}
		trees_count={pageState.data.user.trees_count}
		updates_count={pageState.data.user.updates_count}
		files_count={pageState.data.user.files_count}
	/>

	<UserHeatMap id={pageState.data.user.id} />
{/if}
