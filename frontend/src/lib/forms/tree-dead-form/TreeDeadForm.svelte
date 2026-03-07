<script lang="ts">
	import { Button, Buttons, Form, AuthWrapper } from '$lib/ui';
	import TreeSheet from '$lib/components/tree/TreeSheet.svelte';
	import ChangeHistory from '$lib/components/tree/ChangeHistory.svelte';
	import { locale } from '$lib/locale';
	import { stateUpdater } from '$lib/actions';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, busy, error, tree, save, close } = stateUpdater(id, 'dead');
</script>

<AuthWrapper>
	<Form onSubmit={save}>
		{#if $error}
			<p>{$error.description}</p>
		{:else if $loading}
			<p>Checking the tree...</p>
		{:else}
			<p>{locale.deadHeader()}</p>

			<TreeSheet tree={$tree} />

			<p>{locale.deadUploadHint()}</p>

			<Buttons>
				<Button onClick={save} disabled={$busy}>{locale.deadConfirm()}</Button>
				<Button type="cancel" onClick={close}>{locale.editCancel()}</Button>
			</Buttons>
		{/if}

		<ChangeHistory {id} name="state" />
	</Form>
</AuthWrapper>

<style>
	p {
		margin: 0;
	}
</style>
