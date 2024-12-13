<script>
	import { page } from '$app/stores';
</script>

<div class="container">
	{#if $page.status == 500}
		<h1>Something went wrong</h1>
		<p>We are sorry, but this page is not working currently. Please try again later.</p>
	{:else if $page.status == 404}
		<h1>Page not found</h1>
		<p>We are sorry, but this page does not exist.</p>
	{:else}
		<h1>Error {$page.status}</h1>
		<p>{$page.error?.message ?? 'The error message is unavailable.'}</p>
	{/if}
</div>

<style>
	.container {
		padding: 0 var(--gap);
	}
</style>
