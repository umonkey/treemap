<script lang="ts">
	import UserHeatMap from '$lib/components/UserHeatMap/index.svelte';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import Tabs from '$lib/components/profile/Tabs.svelte';
	import { locale } from '$lib/locale';
	import SignInButton from '$lib/ui/sign-in-button/SignInButton.svelte';
	import ProfileHeader from './ProfileHeader.svelte';
	import { pageState } from './page.svelte';

	$effect(() => {
		pageState.reload();
	});
</script>

<Dialog title={locale.profileTitle()}>
	<Tabs active="profile" />

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
			name={pageState.data.name}
			userpic={pageState.data.picture}
			trees_count={pageState.data.trees_count}
			updates_count={pageState.data.updates_count}
			files_count={pageState.data.files_count}
		/>

		<UserHeatMap id={pageState.data.id} />
	{/if}
</Dialog>
