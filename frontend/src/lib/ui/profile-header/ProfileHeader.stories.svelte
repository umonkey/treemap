<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { ProfileHeader } from '$lib/ui';
	import CAT from '$lib/assets/cat.jpeg';

	const { Story } = defineMeta({
		title: 'UI/ProfileHeader',
		component: ProfileHeader,
		argTypes: {
			name: { control: 'text' },
			userpic: { control: 'text' },
			trees_count: { control: 'number' },
			updates_count: { control: 'number' },
			files_count: { control: 'number' }
		}
	});
</script>

<Story
	name="Primary"
	args={{
		name: 'John Doe',
		userpic: CAT,
		trees_count: 1693,
		updates_count: 154,
		files_count: 3124
	}}
/>
