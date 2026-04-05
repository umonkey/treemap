<script lang="ts">
	import { page } from '$app/stores';
	import ReplaceForm from './ReplaceForm.svelte';

	const id = $derived($page.params.id as string);
</script>

<ReplaceForm {id} />
