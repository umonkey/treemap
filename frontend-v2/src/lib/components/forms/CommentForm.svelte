<script>
	import { isAuthenticated } from '$lib/stores/auth';
	import SignIn from '$lib/components/auth/SignIn.svelte';
</script>

{#if $isAuthenticated}
	<p>Would you like to add a comment?</p>
{:else}
	<p>You need to be authenticated to add comments.</p>
	<SignIn />
{/if}
