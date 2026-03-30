<script lang="ts">
	import { page } from '$app/stores';
	import { rewriteHash } from '$lib/utils/rewrite';

	// Redirect old urls from v1.
	rewriteHash($page.url.hash);
</script>

<svelte:head>
	<title>Trees of Yerevan</title>
</svelte:head>
