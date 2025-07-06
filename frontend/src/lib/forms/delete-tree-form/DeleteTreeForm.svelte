<script lang="ts">
	import {
		Button,
		Buttons,
		Form,
		TreeSheet,
		FilteredChangeList,
		AuthWrapper,
		CommentInput
	} from '$lib/ui';
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
	<Form>
		{#if $error}
			<p>{$error.description}</p>
		{:else if $loading}
			<p>Checking the tree...</p>
		{:else}
			<p>{locale.deleteHeader()}</p>

			<TreeSheet tree={$tree} />

			<p>{locale.deleteUploadHint()}</p>

			<CommentInput value={comment} onChange={handleCommentChange} />

			<Buttons>
				<Button onClick={handleSave} disabled={$busy}>{locale.deleteConfirm()}</Button>
				<Button type="cancel" onClick={close}>{locale.editCancel()}</Button>
			</Buttons>

			<FilteredChangeList changes={$history} name="state" />
		{/if}
	</Form>
</AuthWrapper>

<style>
	p {
		margin: 0;
	}
</style>
