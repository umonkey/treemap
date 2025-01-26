<script>
	import { isAuthenticated } from '$lib/stores/authStore';
	import { locale } from '$lib/locale';

	import SignIn from '$lib/components/auth/SignIn.svelte';
	import CommentInput from '$lib/components/forms/CommentInput.svelte';
	import Button from '$lib/components/forms/Button.svelte';

	const { onSubmit } = $props();

	let message = $state('');

	const onButtonClicked = () => {
		onSubmit(message);
	};
</script>

{#if $isAuthenticated}
	<p>{locale.commentPrompt()}</p>
	<div class="form">
		<CommentInput bind:value={message} />

		<div class="buttons">
			<Button onClick={onButtonClicked} label={locale.commentSubmit()} disabled={!message} />
		</div>
	</div>
{:else}
	<p>{locale.commentSignIn()}</p>
	<SignIn />
{/if}
