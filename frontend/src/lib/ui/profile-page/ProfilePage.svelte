<script lang="ts">
	import { Header, NarrowPage, SignInButton, ProfileHeader, TabList } from '$lib/ui';
	import { loadMe } from '$lib/hooks';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import UserHeatMap from '$lib/components/UserHeatMap/index.svelte';

	const { loading, error, data, statusCode, reload } = loadMe();

	$effect(() => {
		reload();
	});

	const tabs = [
		{ title: locale.profileTitle(), link: routes.profile(), active: true },
		{ title: locale.uploadsTitle(), link: routes.uploads() }
	];
</script>

<svelte:head>
	<title>{locale.profileTitle()}</title>
</svelte:head>

<Header title={locale.profileTitle()} />

<TabList items={tabs} />

<NarrowPage>
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
</NarrowPage>
