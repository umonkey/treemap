<script lang="ts">
	import { locale } from '$lib/locale';
	import { isAuthenticated } from '$lib/stores/authStore';
	import SignIn from '$lib/components/auth/SignIn.svelte';
	import { Button, CommentInput } from '$lib/ui';

	const { onSubmit } = $props<{
		onSubmit: (message: string) => void;
	}>();

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
