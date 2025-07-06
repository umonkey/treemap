<script lang="ts">
	import { Button, Buttons, TreeSheet, FilteredChangeList, AuthWrapper, CommentInput } from '$lib/ui';
	import { locale } from '$lib/locale';
	import { stateUpdater } from '$lib/actions';

	const { id } = $props<{
		id: string;
	}>();

	const { loading, busy, error, history, tree, save, close } = stateUpdater(id, 'gone');
	
	let comment = $state('');
	
	const handleCommentChange = (value: string) => {
		comment = value;
	};
	
	const handleSave = () => {
		save(comment.trim() || undefined);
	};
</script>

<AuthWrapper>
	<div class="delete-tree">
		{#if $error}
			<p>{$error.description}</p>
		{:else if $loading}
			<p>Checking the tree...</p>
		{:else}
			<p>{locale.deleteHeader()}</p>

			<TreeSheet tree={$tree} />

			<p>{locale.deleteUploadHint()}</p>

			<div class="comment-section">
				<CommentInput value={comment} onChange={handleCommentChange} />
			</div>

			<Buttons>
				<Button onClick={handleSave} disabled={$busy}>{locale.deleteConfirm()}</Button>
				<Button type="cancel" onClick={close}>{locale.editCancel()}</Button>
			</Buttons>

			<FilteredChangeList changes={$history} name="state" />
		{/if}
	</div>
</AuthWrapper>

<style>
	.comment-section {
		margin: 1rem 0;
		padding: 1rem;
		border: 1px solid #e0e0e0;
		border-radius: 4px;
		background-color: #f9f9f9;
	}
</style>
