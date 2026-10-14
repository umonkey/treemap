<script lang="ts">
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';

	const { children } = $props();
</script>

<AuthWrapper permission="user:manage">
	{@render children()}
</AuthWrapper>
