<script lang="ts">
	import { isAuthenticated } from '$lib/stores/authStore';
	import SignInButton from '../sign-in-button/SignInButton.svelte';

	const { children } = $props();
</script>

{#if $isAuthenticated}
	{@render children()}
{:else}
	<p>You need to sign in to access this page.</p>
	<SignInButton />
{/if}
