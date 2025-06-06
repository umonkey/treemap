import { get, writable } from 'svelte/store';
import L from 'leaflet';
import { type MountFn } from '$lib/types';
import { getMap } from '$lib/map';

type CallbackFn = () => void;

export const hooks = ({
	onMount,
	onClick,
	icon,
	position
}: {
	onMount: MountFn;
	onClick: CallbackFn;
	icon: string;
	position: string;
}) => {
	// The button control, remove on unmount.
	const button = writable<L.Control | null>(null);

	// Store the image incase we need to update the icon on the fly.
	const buttonImage = writable<HTMLImageElement | null>(null);

	// Adds the button to the map.
	const initialize = () => {
		const container = L.DomUtil.create('div', 'leaflet-bar leaflet-control');

		const button = L.DomUtil.create('a', 'customMapButton', container);

		button.href = '#';
		button.type = 'button';

		const image = L.DomUtil.create('img', 'customMapButtonIcon', button);
		image.src = icon;

		// Save the image reference in case we need to change the icon later.
		buttonImage.set(image);

		L.DomEvent.disableClickPropagation(button);

		// Clicking the button starts the process of adding a row of trees.
		L.DomEvent.on(button, 'click', () => onClick());

		return container;
	};

	onMount(() => {
		const map = getMap();

		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		(L.Control as any).RowButton = L.Control.extend({
			options: {
				position // 'topleft', 'topright', 'bottomleft', or 'bottomright'
			},

			onAdd: initialize
		});

		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const control = new (L.Control as any).RowButton();
		control.addTo(map);

		button.set(control);

		// Remove the button on component reload.
		return () => {
			get(button)?.remove();
		};
	});

	const handleImageChange = (value: string) => {
		console.debug(`[map] Button image changed to ${value}`);
		get(buttonImage)?.setAttribute('src', value);
	};

	const handlePositionChange = (value: string) => {
		console.debug(`[map] Button position changed to ${value}`);
		get(button)?.setPosition(value);
	};

	const handleActiveChange = (value: boolean) => {
		console.debug(`[map] Button active state changed to ${value}`);

		const em = get(button)?.getContainer()?.firstChild as HTMLAnchorElement | null;

		if (value) {
			em?.classList.add('active');
		} else {
			em?.classList.remove('active');
		}
	};

	return { handleImageChange, handlePositionChange, handleActiveChange };
};
