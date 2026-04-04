<script lang="ts">
	import UserIcon from '$lib/icons/UserIcon.svelte';

	const {
		src,
		alt = 'userpic',
		class: className
	} = $props<{
		src?: string | null;
		alt?: string;
		class?: string;
	}>();

	let error = $state(false);

	const handleError = () => {
		error = true;
	};
</script>

<div class="userpic">
	{#if src && !error}
		<img {src} {alt} class={className} referrerpolicy="no-referrer" onerror={handleError} />
	{:else}
		<div class={className}>
			<UserIcon />
		</div>
	{/if}
</div>

<style>
	.userpic {
		img {
			object-fit: cover;
			display: block;
		}

		div {
			display: flex;
			align-items: center;
			justify-content: center;
		}
	}
</style>
