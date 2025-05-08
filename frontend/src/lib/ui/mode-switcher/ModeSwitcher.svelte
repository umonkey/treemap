<script lang="ts">
	import { ModeEnum } from '$lib/enums';
	import { modeStore } from '$lib/stores/modeStore';
	import { goto, routes } from '$lib/routes';
	import { locale } from '$lib/locale';
	import MAPPER from '$lib/assets/mapper-mode-500px.jpeg';
	import EXPLORER from '$lib/assets/explorer-mode-500px.jpeg';
	import REPORTS from '$lib/assets/reports-mode-500px.jpeg';
	import LEARNING from '$lib/assets/learning-mode-500px.jpeg';

	const handleExplorer = (e: Event) => {
		e.preventDefault();
		modeStore.set(ModeEnum.Explorer);
		goto(routes.map());
	};
</script>

<div class="mode-switcher">
	<h2>{locale.chooseFocus()}</h2>

	<div class="grid">
		<a href={routes.modeMapper()}>
			<img src={MAPPER} alt="Mapper Mode" />
			<p>Mapper</p>
		</a>
		<a href={routes.map()} onclick={handleExplorer}>
			<img src={EXPLORER} alt="Explorer Mode" />
			<p>Explorer</p>
		</a>
		<a href={routes.stats()}>
			<img src={REPORTS} alt="Reports Mode" />
			<p>Researcher</p>
		</a>
		<a href={routes.learn()}>
			<img src={LEARNING} alt="Learning Mode" />
			<p>Beginner</p>
		</a>
	</div>
</div>

<style>
	.mode-switcher {
		max-width: var(--narrow-width);

		h2 {
			margin-top: 40px;
			font-weight: 400;
			font-size: 20px;
			opacity: 0.75;
		}
	}

	.grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		grid-gap: var(--gap);

		& > a {
			width: 100%;
			line-height: 0;
			display: block;
			border-radius: 10px;
			overflow: hidden;
			position: relative;
			color: #fff;
		}

		img {
			max-width: 100%;
			aspect-ratio: 1/1;
		}

		p {
			position: absolute;
			bottom: 0;
			left: 0;
			background-color: rgba(0, 0, 0, 0.6);
			width: 100%;
			margin: 0;
			padding: 0;
			line-height: 40px;
			text-align: center;

			color: white;
			font-weight: 800;
			text-transform: uppercase;
		}
	}
</style>
