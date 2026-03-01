<script lang="ts">
	import { uploadStore } from '$lib/stores/upload';
	import { locale } from '$lib/locale';
	import { CheckInput } from '$lib/ui';

	const onChange = (value: boolean) => {
		uploadStore.update((store) => {
			store.autoupload = value;
			return store;
		});
	};
</script>

<CheckInput value={$uploadStore.autoupload} label={locale.uploadsAuto()} {onChange} />
