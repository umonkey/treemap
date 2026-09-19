<script lang="ts">
	import UserPic from '$lib/components/layout/UserPic.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import SignOutButton from './SignOutButton.svelte';

	const { name, userpic } = $props<{
		name: string;
		userpic: string;
	}>();
</script>

<div class="container">
	<div class="header">
		<UserPic src={userpic} alt="userpic" class="userpic-header" />

		<div class="data">
			<h1>Welcome, {name}</h1>

			<Buttons>
				<SignOutButton />
			</Buttons>
		</div>
	</div>
</div>

<style>
	.header {
		display: flex;
		flex-direction: row;
		gap: 1rem;
	}

	:global(.userpic-header) {
		width: 90px;
		height: 90px;
		border-radius: 50%;
		background-color: #000;
		color: transparent; /* hide alt */
		display: block;
		box-shadow: 2px 2px 5px 0px rgba(0, 0, 0, 1);
		z-index: 2;
		object-position: center;
		object-fit: cover;
	}

	.container {
		position: relative;
		z-index: 2;
	}

	h1 {
		font-weight: 800;
		font-size: 24px;
		margin: 0;
	}

	.data {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	@media (max-width: 1023px) {
		h1 {
			font-size: 1.25rem;
			margin: 0;
		}
	}
</style>
