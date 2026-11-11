<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { fn } from '@storybook/test';
	import CommentForm from './CommentForm.svelte';

	const { Story } = defineMeta({
		title: 'UI/CommentForm',
		component: CommentForm,
		args: {
			entityType: 'tree',
			entityId: '1',
			onSubmitted: fn()
		}
	});
</script>

<Story name="Primary" />
