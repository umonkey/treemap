import { mount, unmount, untrack } from 'svelte';
import type { PanoramaImage, PanoramaHint } from '$lib/api/panoramas';
import TreeIcon from '$lib/icons/TreeIcon.svelte';
import 'pannellum';

class PanoramaViewerLogic {
	viewer: Pannellum.Viewer | null = null;
	yaw = $state(0);
	onMove?: (angle: number) => void;
	trees = $state<PanoramaHint[]>([]);
	isLoaded = $state(false);
	private addedHotspotIds: string[] = [];
	private mountedIcons = new Map<string, Record<string, unknown>>();

	init = (
		container: HTMLElement,
		image: PanoramaImage,
		initialYaw: number = 0,
		onMove?: (angle: number) => void
	) => {
		this.unmountIcons();

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

	setTrees = (trees: PanoramaHint[]) => {
		this.trees = trees;

		if (this.viewer && this.isLoaded) {
			console.log('[PanoramaViewer] Updating markers, count:', trees.length);

			// 1. Remove all tracked hotspots
			for (const id of this.addedHotspotIds) {
				this.viewer.removeHotSpot(id);
			}
			this.addedHotspotIds = [];
			this.unmountIcons();

			// 2. Extra safety: remove anything else that might have stuck around
			// Pannellum internally uses a list that can be accessed via getConfig().hotSpots
			const config = this.viewer.getConfig();
			if (config.hotSpots) {
				// Create a copy to avoid mutation issues during iteration
				const current = [...config.hotSpots];
				for (const hs of current) {
					// Check for ID or class to identify our markers
					if (hs.id && (hs.id.startsWith('tree-') || hs.cssClass?.startsWith('tree-marker'))) {
						this.viewer.removeHotSpot(hs.id);
					}
				}
			}

			// 3. Add new hotspots
			for (let i = 0; i < trees.length; i++) {
				const id = `tree-${i}-${Math.random().toString(36).substr(2, 9)}`;
				this.addedHotspotIds.push(id);
				if (trees[i].tree_id) {
					// Auto-generated tree pointer: green disc with a white tree icon,
					// displayed at horizon level.
					this.viewer.addHotSpot({
						id,
						pitch: 0,
						yaw: trees[i].angle,
						type: 'info',
						cssClass: 'tree-marker-disc',
						createTooltipFunc: (div) => {
							const instance = mount(TreeIcon, { target: div });
							this.mountedIcons.set(id, instance);
						}
					});
				} else {
					// Manual hint: vertical line marker.
					this.viewer.addHotSpot({
						id,
						pitch: 0,
						yaw: trees[i].angle,
						type: 'info',
						text: 'Tree',
						cssClass: 'tree-marker'
					});
				}
			}

			// 4. Force a resize/refresh if cleared to ensure UI updates
			if (trees.length === 0) {
				this.viewer.resize();
			}
		}
	};

	private unmountIcons = () => {
		for (const instance of this.mountedIcons.values()) {
			void unmount(instance);
		}
		this.mountedIcons.clear();
	};

	destroy = () => {
		this.unmountIcons();
		if (this.viewer) {
			this.viewer.destroy();
			this.viewer = null;
		}
		this.addedHotspotIds = [];
		this.isLoaded = false;
	};
}

export const componentState = new PanoramaViewerLogic();
