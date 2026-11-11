<script lang="ts">
	import { locale } from '$lib/locale';
	import { hasPermission, isAuthenticated } from '$lib/stores/authStore';
	import type { CommentEntityType } from '$lib/types';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import SignInButton from '$lib/ui/sign-in-button/SignInButton.svelte';
	import CommentInput from './CommentInput.svelte';
	import { CommentFormLogic } from './CommentForm.svelte.ts';

	const {
		entityType,
		entityId,
		onSubmitted
	}: {
		entityType: CommentEntityType;
		entityId: string;
		onSubmitted?: () => void;
	} = $props();

	const formState = new CommentFormLogic();
	const canWrite = $derived($hasPermission('comment:create'));

	const handleSubmit = async () => {
		if (!canWrite) {
			return;
		}

		if (await formState.submit(entityType, entityId)) {
			onSubmitted?.();
		}
	};
</script>

{#if $isAuthenticated}
	<Form onSubmit={handleSubmit}>
		<CommentInput
			value={formState.message}
			onChange={formState.handleChange}
			disabled={!canWrite}
		/>

		<Buttons>
			<Button type="submit" disabled={!canWrite || !formState.message || formState.submitting}
				>{locale.commentSubmit()}</Button
			>
		</Buttons>

		{#if !canWrite}
			<p>{locale.commentNoPermission()}</p>
		{/if}
	</Form>
{:else}
	<p>{locale.commentSignIn()}</p>
	<SignInButton />
{/if}
