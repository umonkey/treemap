<script lang="ts">
	import { LocationInput, Button, Buttons, Form, FilteredChangeList } from '$lib/ui';
	import { locale } from '$lib/locale';
	import { editor } from './hooks';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, busy, loadError, saveError, value, history, save, close, handleChange } =
		editor(id);
</script>

{#if $loading}
	<!-- loading -->
{:else if $loadError}
	<p>{$loadError}</p>
{:else}
	<Form onSubmit={save}>
		<LocationInput value={$value} onChange={handleChange} open />

		<Buttons>
			<Button type="submit" onClick={save} disabled={$busy}>{locale.editSave()}</Button>
			<Button type="cancel" onClick={close}>{locale.editCancel()}</Button>
		</Buttons>
	</Form>

	{#if $saveError}
		<p>{$saveError}</p>
	{/if}

	<FilteredChangeList changes={$history} name="location" />
{/if}
