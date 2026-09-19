<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import ProfileHeader from './ProfileHeader.svelte';
	import CAT from '$lib/assets/cat.jpeg';

	const { Story } = defineMeta({
		title: 'UI/ProfileHeader',
		component: ProfileHeader,
		argTypes: {
			name: { control: 'text' },
			userpic: { control: 'text' }
		}
	});
</script>

<Story
	name="Primary"
	args={{
		name: 'John Doe',
		userpic: CAT
	}}
/>
