<script lang="ts">
	import ChangeHistory from '$lib/components/tree/ChangeHistory.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import { locale } from '$lib/locale';
	import { AuthWrapper, Button, Buttons, CircumferenceInput, Form } from '$lib/ui';
	import { editor } from './hooks';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, loadError, saveError, busy, tree, value, save, close, handleChange } =
		editor(id);
</script>

<AuthWrapper>
	{#if $loading}
		<!-- loading -->
	{:else if $loadError}
		<p>{$loadError}</p>
	{:else if $tree}
		<Title title={$tree.species} address={$tree.address ?? undefined} />
		<TreeContextMenu id={$tree.id} />

		<Form onSubmit={save} sticky>
			<CircumferenceInput value={$value} autofocus onChange={handleChange} />

			<Buttons sticky>
				<Button type="submit" onClick={save} disabled={$busy}>{locale.editSave()}</Button>
				<Button type="cancel" onClick={close}>{locale.editCancel()}</Button>
			</Buttons>

			{#if $saveError}
				<p>{$saveError}</p>
			{/if}

			<ChangeHistory {id} name="circumference" />
		</Form>
	{/if}
</AuthWrapper>
