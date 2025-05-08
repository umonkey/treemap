<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import NarrowPage from './NarrowPage.svelte';

	const { Story } = defineMeta({
		title: 'UI/NarrowPage',
		component: NarrowPage,
		argTypes: {
			title: { control: 'text' },
			nopadding: { control: 'boolean' }
		}
	});
</script>

<Story
	name="Primary"
	args={{
		title: 'This is the title'
	}}
>
	<p>Sample page contents.</p>
</Story>
