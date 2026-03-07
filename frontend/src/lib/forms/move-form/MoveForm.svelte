<script lang="ts">
	import { LocationInput, Button, Buttons, Form } from '$lib/ui';
	import ChangeHistory from '$lib/components/tree/ChangeHistory.svelte';
	import { locale } from '$lib/locale';
	import { editor } from './hooks';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, busy, loadError, saveError, value, save, close, handleChange } = editor(id);
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

	<ChangeHistory {id} name="location" />
{/if}
