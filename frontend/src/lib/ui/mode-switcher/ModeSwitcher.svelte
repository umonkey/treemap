<script lang="ts">
	import { ModeEnum } from '$lib/enums';
	import { modeStore } from '$lib/stores/modeStore';
	import { goto, routes } from '$lib/routes';
	import { locale } from '$lib/locale';

	// Icons come from https://www.freepik.com/
	import EXPLORER from '$lib/icons/explore.png';
	import UPDATE_ICON from '$lib/icons/update-map.png';
	import CHARTS_ICON from '$lib/icons/data-charts.png';
	import LEARN_ICON from '$lib/icons/learn.png';
	import PLANT_ICON from '$lib/icons/plant.png';
	import CAMERA_ICON from '$lib/icons/camera.png';
	import TAPE_ICON from '$lib/icons/tape-measure.png';

	const handleExplorer = (e: Event) => {
		e.preventDefault();
		modeStore.set(ModeEnum.Explorer);
		goto(routes.map());
	};
</script>

<div class="mode-switcher">
	<h2>{locale.chooseFocus()}</h2>

	<div class="grid">
		<div>
			<a href={routes.modeMapper()}>
				<img src={UPDATE_ICON} alt="Tape measure" />
			</a>
			<p>Update Map</p>
		</div>
		<div>
			<a href={routes.searchQuery('gone')}>
				<img src={PLANT_ICON} alt="Plant Trees" />
			</a>
			<p>Plant Trees</p>
		</div>
		<div>
			<a href={routes.searchQuery('noimage')}>
				<img src={CAMERA_ICON} alt="Photo camera" />
			</a>
			<p>Take Photos</p>
		</div>
		<div>
			<a href={routes.searchQuery('noimage')}>
				<img src={TAPE_ICON} alt="Tape measure" />
			</a>
			<p>Measure Trees</p>
		</div>
		<div>
			<a href={routes.map()} onclick={handleExplorer}>
				<img src={EXPLORER} alt="Explorer Mode" />
			</a>
			<p>Explore Map</p>
		</div>
		<div>
			<a href={routes.stats()}>
				<img src={CHARTS_ICON} alt="Reports Mode" />
			</a>
			<p>Analyze</p>
		</div>
		<div>
			<a href={routes.learn()}>
				<img src={LEARN_ICON} alt="Learning Mode" />
			</a>
			<p>Learn Trees</p>
		</div>
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
		grid-template-columns: repeat(4, 1fr);
		grid-gap: 40px;

		& > a {
			width: 100%;
			line-height: 0;
			display: block;
			position: relative;
			color: #fff;
			text-decoration: none;
		}

		img {
			background-color: light-dark(#ccc, #eee);
			border-radius: 30px;
			overflow: hidden;
			max-width: 100%;
			aspect-ratio: 1/1;
		}

		p {
			width: 100%;
			margin: 0;
			padding: 0;
			font-size: 14px;
			line-height: 20px;
			text-align: center;

			color: white;
			font-weight: 300;
		}
	}

	@media screen and (max-width: 640px) {
		.grid {
			gap: 10px;
		}

		p {
			letter-spacing: -1px;
		}
	}
</style>
