<script lang="ts">
	import SignInButton from '$lib/ui/sign-in-button/SignInButton.svelte';
	import ProfileHeader from './ProfileHeader.svelte';
	import { loadMe } from '$lib/hooks';
	import { locale } from '$lib/locale';
	import UserHeatMap from '$lib/components/UserHeatMap/index.svelte';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import Tabs from '$lib/components/profile/Tabs.svelte';

	const { loading, error, data, statusCode, reload } = loadMe();

	$effect(() => {
		reload();
	});
</script>

<Dialog title={locale.profileTitle()}>
	<Tabs active="profile" />

	{#if $loading}
		...
	{:else if $statusCode === 401}
		<div class="container signedOut">
			<p>{locale.profileSignInPrompt()}</p>
			<SignInButton />
		</div>
	{:else if $error}
		<p>{$error.description}</p>
	{:else if $data}
		<ProfileHeader
			name={$data.name}
			userpic={$data.picture}
			trees_count={$data.trees_count}
			updates_count={$data.updates_count}
			files_count={$data.files_count}
		/>

		<UserHeatMap id={$data.id} />
	{/if}
</Dialog>
