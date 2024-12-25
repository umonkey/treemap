<script>
	import { isAuthenticated } from '$lib/stores/auth';

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
	<p>Would you like to add a comment?</p>
	<div class="form">
		<CommentInput bind:value={message} />

		<div class="buttons">
			<Button onClick={onButtonClicked} label="Submit comment" disabled={!message} />
		</div>
	</div>
{:else}
	<p>You need to be authenticated to add comments.</p>
	<SignIn />
{/if}
