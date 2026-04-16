<script lang="ts">
	import { page } from '$app/stores';
	import TreeDeadForm from './TreeDeadForm.svelte';

	const id = $derived($page.params.id as string);
</script>

<TreeDeadForm {id} />
