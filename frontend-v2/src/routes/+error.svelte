<script>
	import { page } from '$app/stores';
</script>

<div class="container">
	<h1>Error {$page.status}</h1>

	{#if $page.status == 500}
		<p>We are sorry, but this page is not working currently. Please try again later.</p>
	{:else}
		<p>{$page.error.message}</p>
	{/if}
</div>

<style>
	.container {
		padding: 0 var(--gap);
	}
</style>
