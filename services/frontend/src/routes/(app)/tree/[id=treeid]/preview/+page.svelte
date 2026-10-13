<script lang="ts">
	// Legacy URL: the tree root page now lives at /tree/:id.
	import { page } from '$app/state';
	import { goto, routes } from '$lib/routes';

	const id = $derived(page.params.id as string);

	$effect(() => {
		if (id) {
			void goto(routes.treeDetails(id), { replaceState: true });
		}
	});
</script>
