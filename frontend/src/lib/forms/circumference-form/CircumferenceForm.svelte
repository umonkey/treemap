<script lang="ts">
	import { CircumferenceInput, Button, Buttons, Form, AuthWrapper } from '$lib/ui';
	import ChangeHistory from '$lib/components/tree/ChangeHistory.svelte';
	import { locale } from '$lib/locale';
	import { editor } from './hooks';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, loadError, saveError, busy, value, save, close, handleChange } = editor(id);
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
				<Button type="submit" onClick={save} disabled={$busy}>{locale.editSave()}</Button>
				<Button type="cancel" onClick={close}>{locale.editCancel()}</Button>
			</Buttons>
		</Form>

		{#if $saveError}
			<p>{$saveError}</p>
		{/if}

		<ChangeHistory {id} name="circumference" />
	{/if}
</AuthWrapper>
