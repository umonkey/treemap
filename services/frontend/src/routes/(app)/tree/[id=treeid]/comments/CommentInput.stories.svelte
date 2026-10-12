<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import CommentInput from './CommentInput.svelte';
	import { fn } from '@storybook/test';

	const { Story } = defineMeta({
		title: 'UI/CommentInput',
		component: CommentInput,
		argTypes: {
			value: { control: 'text' }
		},
		args: {
			onChange: fn()
		}
	});
</script>

<Story
	name="Primary"
	args={{
		value: 'no comment'
	}}
/>
