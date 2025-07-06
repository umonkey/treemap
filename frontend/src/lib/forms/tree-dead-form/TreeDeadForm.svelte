<script lang="ts">
	import { Button, Buttons, Form, TreeSheet, FilteredChangeList, AuthWrapper } from '$lib/ui';
	import { locale } from '$lib/locale';
	import { stateUpdater } from '$lib/actions';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, busy, error, history, tree, save, close } = stateUpdater(id, 'dead');
</script>

<AuthWrapper>
	<Form>
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

		<FilteredChangeList changes={$history} name="state" />
	</Form>
</AuthWrapper>

<style>
	p {
		margin: 0;
	}
</style>
