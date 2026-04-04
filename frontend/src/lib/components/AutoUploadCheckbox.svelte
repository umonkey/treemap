<script lang="ts">
	import { locale } from '$lib/locale';
	import { uploadStore } from '$lib/stores/upload';
	import CheckInput from '$lib/ui/check-input/CheckInput.svelte';

	const onChange = (value: boolean) => {
		uploadStore.update((store) => {
			store.autoupload = value;
			return store;
		});
	};
</script>

<CheckInput value={$uploadStore.autoupload} label={locale.uploadsAuto()} {onChange} />
