<script lang="ts">
	import SignInButton from '$lib/ui/sign-in-button/SignInButton.svelte';
	import ProfileHeader from './ProfileHeader.svelte';
	import TabList from '$lib/ui/tab-list/TabList.svelte';
	import LayerSelector from '$lib/components/map/LayerSelector.svelte';
	import { loadMe } from '$lib/hooks';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import UserHeatMap from '$lib/components/UserHeatMap/index.svelte';
	import { uploadStore } from '$lib/stores/upload';
	import Dialog from '$lib/components/layout/Dialog.svelte';

	const { loading, error, data, statusCode, reload } = loadMe();

	$effect(() => {
		reload();
	});

	const tabs = $derived([
		{ title: locale.profileTitle(), link: routes.profile(), active: true },
		{
			title: locale.uploadsTitle(),
			link: routes.uploads(),
			badge: $uploadStore.pending > 0 ? $uploadStore.pending : undefined
		}
	]);
</script>

<Dialog title={locale.profileTitle()}>
	<TabList items={tabs} />

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

		<LayerSelector />

		<UserHeatMap id={$data.id} />
	{/if}
</Dialog>
