<script lang="ts">
	import { SignInButton } from '$lib/ui';
	import { isAuthenticated } from '$lib/stores/authStore';

	const { children } = $props();
</script>

{#if $isAuthenticated}
	{@render children()}
{:else}
	<p>You need to sign in to access this page.</p>
	<SignInButton />
{/if}
