<script lang="ts">
	import { locale } from '$lib/locale';
	import { isAuthenticated } from '$lib/stores/authStore';
	import SignIn from '$lib/components/auth/SignIn.svelte';
	import { Button, Buttons, CommentInput } from '$lib/ui';

	const { onSubmit } = $props<{
		onSubmit: (message: string) => void;
	}>();

	let message = $state('');

	const onButtonClicked = () => {
		onSubmit(message);
	};

	const handleChange = (value: string) => {
		message = value;
	};
</script>

{#if $isAuthenticated}
	<p>{locale.commentPrompt()}</p>
	<div class="form">
		<CommentInput value={message} onChange={handleChange} />

		<Buttons>
			<Button onClick={onButtonClicked} label={locale.commentSubmit()} disabled={!message} />
		</Buttons>
	</div>
{:else}
	<p>{locale.commentSignIn()}</p>
	<SignIn />
{/if}
