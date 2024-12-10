<script>
	import { authState } from '$lib/stores/auth';
	import SignIn from '$lib/components/auth/SignIn.svelte';
</script>

<h1>Profile</h1>

{#if authState}
	<p>You are logged in as <em>{$authState.name}</em>.</p>
{:else}
	<SignIn />
{/if}
