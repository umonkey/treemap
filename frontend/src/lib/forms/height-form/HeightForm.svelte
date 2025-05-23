<script lang="ts">
	import AuthWrapper from '$lib/components/auth/AuthWrapper.svelte';
	import { HeightInput, Button, Buttons, Form, FilteredChangeList } from '$lib/ui';
	import { locale } from '$lib/locale';
	import { editor } from './hooks';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, busy, loadError, saveError, value, history, save, close, handleChange } =
		editor(id);
</script>

<AuthWrapper>
	{#if $loading}
		<!-- loading -->
	{:else if $loadError}
		<p>{$loadError}</p>
	{:else}
		<Form onSubmit={save}>
			<HeightInput value={$value} autofocus onChange={handleChange} />

			<Buttons>
				<Button label={locale.editSave()} type="submit" onClick={save} disabled={$busy} />
				<Button label={locale.editCancel()} type="cancel" onClick={close} />
			</Buttons>
		</Form>

		{#if $saveError}
			<p>{$saveError}</p>
		{/if}

		<FilteredChangeList changes={$history} name="height" />
	{/if}
</AuthWrapper>
