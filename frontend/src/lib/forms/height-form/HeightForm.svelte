<script lang="ts">
	import { HeightInput, Button, Buttons, Form, AuthWrapper } from '$lib/ui';
	import ChangeHistory from '$lib/components/tree/ChangeHistory.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import { locale } from '$lib/locale';
	import { editor } from './hooks';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, busy, loadError, saveError, tree, value, save, close, handleChange } =
		editor(id);
</script>

<AuthWrapper>
	{#if $loading}
		<!-- loading -->
	{:else if $loadError}
		<p>{$loadError}</p>
	{:else}
		<Title title={$tree?.species} address={$tree?.address} />
		<TreeContextMenu id={$tree.id} />

		<Form onSubmit={save}>
			<HeightInput value={$value} autofocus onChange={handleChange} />

			<Buttons>
				<Button type="submit" onClick={save} disabled={$busy}>{locale.editSave()}</Button>
				<Button type="cancel" onClick={close}>{locale.editCancel()}</Button>
			</Buttons>

			{#if $saveError}
				<p>{$saveError}</p>
			{/if}

			<ChangeHistory {id} name="height" />
		</Form>
	{/if}
</AuthWrapper>
