<script lang="ts">
	import { uploadStore } from '$lib/stores/upload';

	const handleChange = (e: Event) => {
		if (e.target) {
			const em = e.target as HTMLInputElement;

			uploadStore.update((store) => {
				store.autoupload = !!em.checked;
				return store;
			});
		}
	};
</script>

<label class="control">
	<input type="checkbox" checked={$uploadStore.autoupload} onchange={handleChange} />
	<span>Auto-upload files</span>
</label>

<style>
	label {
		cursor: pointer;
	}
</style>
