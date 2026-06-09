import { untrack } from 'svelte';
import type { MapillaryImage, MapillaryTree } from '$lib/api/mapillary';
import 'pannellum';

class PanoramaViewerLogic {
	viewer: Pannellum.Viewer | null = null;
	yaw = $state(0);
	onMove?: (angle: number) => void;
	trees = $state<MapillaryTree[]>([]);
	isLoaded = $state(false);
	private addedHotspotIds: string[] = [];

	init = (
		container: HTMLElement,
		image: MapillaryImage,
		initialYaw: number = 0,
		onMove?: (angle: number) => void
	) => {
		if (this.viewer) {
			this.viewer.destroy();
			this.viewer = null;
		}

		this.isLoaded = false;
		this.addedHotspotIds = [];
		this.yaw = initialYaw;
		this.onMove = onMove;

		if (!image.url) return;

		this.viewer = window.pannellum.viewer(container, {
			type: 'equirectangular',
			panorama: image.url,
			autoLoad: true,
			compass: true,
			northOffset: image.compass_angle,
			yaw: initialYaw,
			showControls: false,
			mouseZoom: true,
			keyboardZoom: true,
			hotSpots: [] // We add them via setTrees once loaded
		});

		this.viewer.on('load', () => {
			console.log('[PanoramaViewer] Loaded');
			this.isLoaded = true;
			this.viewer?.resize();
			this.onMove?.(this.yaw);
			// Apply trees now that the viewer is ready
			this.setTrees(untrack(() => this.trees));
		});

		this.viewer.on('animatefinished', (data) => {
			this.yaw = data.yaw;
			this.onMove?.(this.yaw);
		});
	};

	setTrees = (trees: MapillaryTree[]) => {
		this.trees = trees;

		if (this.viewer && this.isLoaded) {
			console.log('[PanoramaViewer] Updating markers, count:', trees.length);

			// 1. Remove all tracked hotspots
			for (const id of this.addedHotspotIds) {
				this.viewer.removeHotSpot(id);
			}
			this.addedHotspotIds = [];

			// 2. Extra safety: remove anything else that might have stuck around
			// Pannellum internally uses a list that can be accessed via getConfig().hotSpots
			const config = this.viewer.getConfig();
			if (config.hotSpots) {
				// Create a copy to avoid mutation issues during iteration
				const current = [...config.hotSpots];
				for (const hs of current) {
					// Check for ID or class to identify our markers
					if (hs.id && (hs.id.startsWith('tree-') || hs.cssClass === 'tree-marker')) {
						this.viewer.removeHotSpot(hs.id);
					}
				}
			}

			// 3. Add new hotspots
			for (let i = 0; i < trees.length; i++) {
				const id = `tree-${i}-${Math.random().toString(36).substr(2, 9)}`;
				this.addedHotspotIds.push(id);
				this.viewer.addHotSpot({
					id,
					pitch: 0,
					yaw: trees[i].angle,
					type: 'info',
					text: 'Tree',
					cssClass: 'tree-marker'
				});
			}

			// 4. Force a resize/refresh if cleared to ensure UI updates
			if (trees.length === 0) {
				this.viewer.resize();
			}
		}
	};

	destroy = () => {
		if (this.viewer) {
			this.viewer.destroy();
			this.viewer = null;
		}
		this.addedHotspotIds = [];
		this.isLoaded = false;
	};
}

export const componentState = new PanoramaViewerLogic();
