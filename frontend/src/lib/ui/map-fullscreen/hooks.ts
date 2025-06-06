import type { MountFn } from '$lib/types';
import type { Map } from 'leaflet';
import { getMap } from '$lib/map';

export const hooks = ({ onMount }: { onMount: MountFn }) => {
	let map: Map;

	const handleClick = () => {
		if (document.fullscreenElement) {
			document.exitFullscreen();
			return;
		}

		const container = map.getContainer().parentElement;

		if (container?.requestFullscreen) {
			container.requestFullscreen();
		}
	};

	onMount(() => {
		map = getMap();
	});

	return { handleClick };
};
