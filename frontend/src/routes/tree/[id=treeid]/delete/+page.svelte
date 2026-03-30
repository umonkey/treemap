<script lang="ts">
	import DeleteTreeForm from './DeleteTreeForm.svelte';

	const { data } = $props<{
		data: {
			id: string;
		};
	}>();
</script>

<DeleteTreeForm id={data.id} />
