<script>
	import TextInput from '$lib/components/forms/TextInput.svelte';
	import { locale } from '$lib/locale';

	export let value = '';
</script>

<TextInput label={locale.notesLabel()} bind:value multiline={true} hint={locale.notesHint()} />
