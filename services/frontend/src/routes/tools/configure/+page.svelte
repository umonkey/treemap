<script lang="ts">
	import { locale as appLocale } from '$lib/locale';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import CheckInput from '$lib/ui/check-input/CheckInput.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import TextInput from '$lib/ui/text-input/TextInput.svelte';
	import { locale } from './lang';
	import { PageState } from './page.svelte';

	const pageState = new PageState();

	$effect(() => {
		pageState.reload();
	});
</script>

<svelte:head>
	<title>{locale.title()} — {appLocale.appTitle()}</title>
</svelte:head>

<article>
	<PageHeader text={locale.title()} />

	{#if pageState.loading}
		<p aria-busy="true">{locale.loading()}</p>
	{:else if pageState.error}
		<p class="error">{locale.error(pageState.error.description)}</p>
	{:else}
		<Form onSubmit={pageState.apply}>
			<TextInput
				label={locale.name()}
				value={pageState.name}
				onChange={(value) => (pageState.name = value)}
			/>

			<TextInput
				label={locale.description()}
				value={pageState.description}
				multiline
				onChange={(value) => (pageState.description = value)}
			/>

			<h3>{locale.osmIntegration()}</h3>

			<CheckInput
				label={locale.osmPull()}
				value={pageState.osmPull}
				onChange={(value) => (pageState.osmPull = value)}
			/>

			<TextInput
				label={locale.osmBotId()}
				value={pageState.osmBotId}
				hint={locale.osmBotIdHint()}
				onChange={(value) => (pageState.osmBotId = value)}
			/>

			<Buttons>
				<Button type="submit" disabled={pageState.saving}>
					{pageState.saving ? locale.applying() : locale.apply()}
				</Button>
			</Buttons>
		</Form>
	{/if}
</article>

<style>
	.error {
		color: red;
	}
</style>
