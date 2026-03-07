<script lang="ts">
	import { Button, Buttons, Form, AuthWrapper, CommentInput } from '$lib/ui';
	import TreeSheet from '$lib/components/tree/TreeSheet.svelte';
	import ChangeHistory from '$lib/components/tree/ChangeHistory.svelte';
	import { locale } from '$lib/locale';
	import { stateUpdater } from '$lib/actions';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, busy, error, tree, save, close, handleCommentChange } = stateUpdater(id, 'gone');
</script>

<AuthWrapper>
	<Form onSubmit={save}>
		{#if $error}
			<p>{$error.description}</p>
		{:else if $loading}
			<p>Checking the tree...</p>
		{:else}
			<p>{locale.deleteHeader()}</p>

			<TreeSheet tree={$tree} />

			<p>{locale.deleteUploadHint()}</p>

			<CommentInput value={''} hint={locale.deleteCommentHint()} onChange={handleCommentChange} />

			<Buttons>
				<Button onClick={save} disabled={$busy}>{locale.deleteConfirm()}</Button>
				<Button type="cancel" onClick={close}>{locale.editCancel()}</Button>
			</Buttons>

			<ChangeHistory {id} name="state" />
		{/if}
	</Form>
</AuthWrapper>

<style>
	p {
		margin: 0;
	}
</style>
