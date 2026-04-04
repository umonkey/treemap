<script lang="ts">
	import { locale } from '$lib/locale';
	import { isAuthenticated } from '$lib/stores/authStore';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import CommentInput from '$lib/ui/comment-input/CommentInput.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import SignInButton from '$lib/ui/sign-in-button/SignInButton.svelte';

	const { onSubmit, authenticated } = $props<{
		onSubmit: (message: string) => void;
		authenticated?: boolean;
	}>();

	let message = $state('');

	const onButtonClicked = () => {
		onSubmit(message);
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
			<Button type="submit" onClick={onButtonClicked} disabled={!message}
				>{locale.commentSubmit()}</Button
			>
		</Buttons>
	</Form>
{:else}
	<p>{locale.commentSignIn()}</p>
	<SignInButton />
{/if}
