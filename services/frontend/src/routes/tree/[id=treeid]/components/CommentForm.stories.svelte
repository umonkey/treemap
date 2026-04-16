<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import CommentForm from './CommentForm.svelte';
	import { fn } from '@storybook/test';

	const { Story } = defineMeta({
		title: 'UI/CommentForm',
		component: CommentForm,
		argTypes: {
			authenticated: { control: 'boolean' }
		},
		args: {
			onSubmit: fn()
		}
	});
</script>

<Story
	name="Anonymous"
	args={{
		authenticated: false
	}}
/>

<Story
	name="Authenticated"
	args={{
		authenticated: true
	}}
/>
