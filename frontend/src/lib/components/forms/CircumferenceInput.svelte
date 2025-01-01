<script lang="ts">
	import { locale } from '$lib/locale';
	import TextInput from '$lib/components/forms/TextInput.svelte';

	// This is the input value in meters.
	let {
		value = $bindable(0)
	}: {
		value: number;
	} = $props();

	// This is the editable value in centimeters.
	let centimeters = $state(value * 100);

	$effect(() => {
		value = centimeters / 100;
	});
</script>

<TextInput
	label={locale.circumferenceLabel()}
	bind:value={centimeters}
	type="number"
	hint={locale.circumferenceHint()}
	help={locale.helpCircumference()}
/>
