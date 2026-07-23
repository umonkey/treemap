import { mapState } from './MapLibre.svelte.ts';
import { showError, showInfo } from '$lib/errors';
import type { Map } from 'maplibre-gl';

export class MapInspectLogic {
	private timer: ReturnType<typeof setTimeout> | null = null;

	private startTimer = () => {
		this.clearTimer();
		this.timer = setTimeout(async () => {
			if (mapState.map) {
				try {
					const center = mapState.map.getCenter();
					const coords = `${center.lat},${center.lng}`;
					await navigator.clipboard.writeText(coords);
					showInfo('Coordinates copied to clipboard.');
				} catch (e) {
					console.error('Failed to copy coordinates to clipboard', e);
					showError('Failed to copy coordinates to clipboard.');
				}
			}
		}, 2000);
	};

	private resetTimer = () => {
		this.startTimer();
	};

	private clearTimer = () => {
		if (this.timer !== null) {
			clearTimeout(this.timer);
			this.timer = null;
		}
	};

	private handleMoveStart = () => {
		this.startTimer();
	};

	private handleMove = () => {
		this.resetTimer();
	};

	private handleMoveEnd = () => {
		this.clearTimer();
	};

	public mount = (map: Map) => {
		map.on('movestart', this.handleMoveStart);
		map.on('move', this.handleMove);
		map.on('moveend', this.handleMoveEnd);
	};

	public unmount = (map: Map) => {
		map.off('movestart', this.handleMoveStart);
		map.off('move', this.handleMove);
		map.off('moveend', this.handleMoveEnd);
		this.clearTimer();
	};
}
