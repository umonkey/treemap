<script lang="ts">
	import { locale } from '$lib/locale';
	import { isAuthenticated } from '$lib/stores/authStore';
	import { Button, Buttons, CommentInput, Form, SignInButton } from '$lib/ui';

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
