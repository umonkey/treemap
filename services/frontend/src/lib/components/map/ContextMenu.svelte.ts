import { getSearchTreesGeoJSON } from '$lib/api/trees';
import { mapBus } from '$lib/buses/mapBus';
import { menuBus, type IMapMenuEvent } from '$lib/buses/menuBus';
import type { IContextMenuPosition } from '$lib/components/menu/BaseContextMenu.svelte.ts';
import { goto } from '$lib/routes';
import { combineQuery, missingQuery } from '$lib/stores/mapLayerStore';
import { searchStore } from '$lib/stores/searchStore';
import type { IBounds, ILatLng } from '$lib/types';
import { get } from 'svelte/store';

export class ContextMenuLogic {
	open = $state(false);

	private coords: ILatLng | undefined = undefined;
	position = $state<IContextMenuPosition | undefined>(undefined);
	private bounds = $state<IBounds | undefined>(undefined);

	downloadUrl = $derived(
		getSearchTreesGeoJSON(
			combineQuery(get(searchStore), get(missingQuery)) ?? '',
			this.bounds?.zoom,
			this.bounds
		)
	);

	private handleShow = (event: IMapMenuEvent) => {
		console.debug(`Showing menu for ${event.lat},${event.lng}`);
		this.coords = { lat: event.lat, lng: event.lng };
		this.position =
			event.x !== undefined && event.y !== undefined ? { x: event.x, y: event.y } : undefined;
		this.open = true;
	};

	private handleBounds = (bounds: IBounds) => {
		this.bounds = bounds;
	};

	public handleClose = () => {
		this.open = false;
	};

	public handleNavigate = async (url: string) => {
		this.open = false;

		if (this.coords) {
			mapBus.emit('move', this.coords);
		}

		await goto(url);
	};

	public handleCopyCoordinates = async () => {
		if (!this.coords) {
			return;
		}

		try {
			await navigator.clipboard.writeText(`${this.coords.lat},${this.coords.lng}`);
		} catch (e) {
			console.error('Failed to copy coordinates to clipboard', e);
		} finally {
			this.open = false;
		}
	};

	public handleDownload = () => {
		this.open = false;
	};

	public subscribe = () => {
		menuBus.on('showMap', this.handleShow);
		mapBus.on('bounds', this.handleBounds);

		return () => {
			menuBus.off('showMap', this.handleShow);
			mapBus.off('bounds', this.handleBounds);
		};
	};
}
