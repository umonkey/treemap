<script lang="ts">
	import { page } from '$app/stores';
	import MoveForm from './MoveForm.svelte';

	const id = $derived($page.params.id as string);
</script>

<MoveForm {id} />
