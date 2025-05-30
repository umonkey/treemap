<script lang="ts">
	import { locale } from '$lib/locale';
	import { Buttons, Button, SignOutButton } from '$lib/ui';
	import HEADER_IMAGE from '$lib/assets/header.jpg';
	import { routes } from '$lib/routes';

	const { name, userpic, trees_count, updates_count, files_count } = $props<{
		name: string;
		userpic: string;
		trees_count: number;
		updates_count: number;
		files_count: number;
	}>();
</script>

<div class="header">
	<img class="header" src={HEADER_IMAGE} alt="header background" />

	<div class="container signedIn">
		<img class="userpic" src={userpic} alt="userpic" />

		<h1>{name}</h1>

		<div class="stats">
			{locale.profileTrees(trees_count)}, {locale.profileUpdates(updates_count)}, {locale.profilePhotos(
				files_count
			)}.
		</div>

		<Buttons>
			<SignOutButton />
			<Button label={locale.settingsButton()} link={routes.settings()} type="secondary" />
		</Buttons>
	</div>
</div>

<style>
	img.userpic {
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
		padding: 0 var(--gap);
		position: relative;
		z-index: 2;

		&.signedIn {
			margin-top: -45px;
		}
	}

	h1 {
		font-weight: 800;
		font-size: 24px;
		margin: 10px 0 5px;
	}

	.stats {
		margin: var(--gap) 0;
	}

	@media (max-width: 480px) {
		img.header {
			height: 40vw;
		}
	}

	@media (min-width: 481px) {
		img.header {
			height: 200px;
		}
	}
</style>
