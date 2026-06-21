<script lang="ts">
	import { locale } from '$lib/locale';
	import { isAuthenticated } from '$lib/stores/authStore';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import SignInButton from '$lib/ui/sign-in-button/SignInButton.svelte';
	import CommentInput from './CommentInput.svelte';

	const { onSubmit, authenticated } = $props<{
		onSubmit: (message: string) => Promise<boolean>;
		authenticated?: boolean;
	}>();

	let message = $state('');
	let submitting = $state(false);

	const onButtonClicked = async () => {
		if (!message || submitting) {
			return;
		}

		submitting = true;
		const success = await onSubmit(message);
		if (success) {
			message = '';
		}
		submitting = false;
	};

	const handleChange = (value: string) => {
		message = value;
	};
</script>

{#if $isAuthenticated || !!authenticated}
	<p>{locale.commentPrompt()}</p>

	<Form>
		<CommentInput value={message} onChange={handleChange} />

		<Buttons>
			<Button
				type="submit"
				onClick={onButtonClicked}
				disabled={!message || submitting}
				>{locale.commentSubmit()}</Button
			>
		</Buttons>
	</Form>
{:else}
	<p>{locale.commentSignIn()}</p>
	<SignInButton />
{/if}
