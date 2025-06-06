import type { Map } from 'leaflet';
import type { ILatLng, MountFn } from '$lib/types';
import { getMap } from '$lib/map';

type ConfirmFn = (center: ILatLng) => void;

export const hooks = ({ onMount, onConfirm }: { onMount: MountFn; onConfirm: ConfirmFn }) => {
	let map: Map;

	const handleClick = () => {
		const center = map.getCenter();
		onConfirm(center);
	};

	onMount(() => {
		map = getMap();
	});

	return { handleClick };
};
