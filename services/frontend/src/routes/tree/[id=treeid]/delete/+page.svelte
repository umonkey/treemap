<script lang="ts">
	import { page } from '$app/stores';
	import DeleteTreeForm from './DeleteTreeForm.svelte';

	const id = $derived($page.params.id as string);
</script>

<DeleteTreeForm {id} />
