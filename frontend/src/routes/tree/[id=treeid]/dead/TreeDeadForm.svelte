<script lang="ts">
	import { stateUpdater } from '$lib/actions';
	import ChangeHistory from '$lib/components/tree/ChangeHistory.svelte';
	import Title from '$lib/components/tree/Title.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import TreeSheet from '$lib/components/tree/TreeSheet.svelte';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { locale } from '$lib/locale';
	import { AuthWrapper, Button, Buttons, Form } from '$lib/ui';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, busy, error, tree, save, close } = stateUpdater(id, 'dead');
</script>

<AuthWrapper>
	{#if $error}
		<p>{$error}</p>
	{:else if $loading}
		<p>Checking the tree...</p>
	{:else if $tree}
		<TreeForm tree={$tree} onSubmit={save} onCancel={close} saving={$busy}>
			<p>{locale.deadHeader()}</p>

			<TreeSheet tree={$tree} />

			<p>{locale.deadUploadHint()}</p>

			<ChangeHistory {id} name="state" />
		</TreeForm>
	{/if}
</AuthWrapper>

<style>
	p {
		margin: 0;
	}
</style>
