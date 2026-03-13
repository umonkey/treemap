<script lang="ts">
	import ChangeHistory from '$lib/components/tree/ChangeHistory.svelte';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { LocationInput } from '$lib/ui';
	import { editor } from './hooks';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, busy, loadError, saveError, tree, value, save, close, handleChange } =
		editor(id);
</script>

{#if $loading}
	<!-- loading -->
{:else if $loadError}
	<p>{$loadError}</p>
{:else if $tree}
	<TreeForm {id} title="Move Tree" onSubmit={save} onCancel={close} saving={$busy}>
		<LocationInput value={$value} onChange={handleChange} open />

		{#if $saveError}
			<p>{$saveError}</p>
		{/if}

		<ChangeHistory {id} name="location" />
	</TreeForm>
{/if}
