<script lang="ts">
	import {
		CircumferenceInput,
		Button,
		Buttons,
		Form,
		FilteredChangeList,
		AuthWrapper
	} from '$lib/ui';
	import { locale } from '$lib/locale';
	import { editor } from './hooks';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, loadError, saveError, busy, value, history, save, close, handleChange } =
		editor(id);
</script>

<AuthWrapper>
	{#if $loading}
		<!-- loading -->
	{:else if $loadError}
		<p>{$loadError}</p>
	{:else}
		<Form onSubmit={save}>
			<CircumferenceInput value={$value} autofocus onChange={handleChange} />

			<Buttons>
				<Button label={locale.editSave()} type="submit" onClick={save} disabled={$busy} />
				<Button label={locale.editCancel()} type="cancel" onClick={close} />
			</Buttons>
		</Form>

		{#if $saveError}
			<p>{$saveError}</p>
		{/if}

		<FilteredChangeList changes={$history} name="circumference" />
	{/if}
</AuthWrapper>
