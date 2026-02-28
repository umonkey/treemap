<script lang="ts">
	import { uploadStore } from '$lib/stores/upload';
	import { locale } from '$lib/locale';

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
	<span>{locale.uploadsAuto()}</span>
</label>

<style>
	.control {
		display: flex;
		align-items: center;
		gap: 8px;
		cursor: pointer;
		margin-bottom: 1rem;
		padding: 8px 0;
		color: var(--text-color);
	}

	input {
		width: 18px;
		height: 18px;
		cursor: pointer;
	}
</style>
