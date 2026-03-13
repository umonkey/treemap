<script lang="ts">
	import ChangeHistory from '$lib/components/tree/ChangeHistory.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { locale } from '$lib/locale';
	import { AuthWrapper, Button, Buttons, CanopyInput, Form } from '$lib/ui';
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
		<TreeForm tree={$tree} onSubmit={save} onCancel={close} saving={$busy}>
			<CanopyInput value={$value} autofocus onChange={handleChange} />

			{#if $saveError}
				<p>{$saveError}</p>
			{/if}

			<ChangeHistory {id} name="diameter" />
		</TreeForm>
	{/if}
</AuthWrapper>
