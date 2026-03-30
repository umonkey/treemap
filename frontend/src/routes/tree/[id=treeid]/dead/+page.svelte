<script lang="ts">
	import TreeDeadForm from './TreeDeadForm.svelte';

	const { data } = $props<{
		data: {
			id: string;
		};
	}>();
</script>

<TreeDeadForm id={data.id} />
