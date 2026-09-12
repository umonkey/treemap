import { mount, unmount, untrack } from 'svelte';
import {
	getPanoramasImageHints,
	addPanoramaImageHint,
	deleteImageHints,
	updatePanoramaImage,
	type PanoramaImage,
	type PanoramaHint
} from '$lib/api/panoramas';
import TreeIcon from '$lib/icons/TreeIcon.svelte';
import CameraIcon from '$lib/icons/CameraIcon.svelte';
import { panoBus } from '$lib/buses/panoBus';
import { showError } from '$lib/errors';
import { goto, routes } from '$lib/routes';
import 'pannellum';

class PanoramaViewerLogic {
	viewer: Pannellum.Viewer | null = null;
	yaw = $state(0);
	onMove?: (angle: number) => void;
	trees = $state<PanoramaHint[]>([]);
	isLoaded = $state(false);
	isBusy = $state(false);
	canEdit = $state(false);
	isHidden = $state(false);
	private currentImageId?: string;
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
		this.isHidden = image.hidden;

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

	handleKeydown = (e: KeyboardEvent) => {
		if (e.ctrlKey || e.metaKey || e.altKey) return;
		const target = e.target as HTMLElement;
		if (['INPUT', 'TEXTAREA', 'BUTTON'].includes(target?.tagName)) return;
		if (e.code === 'Space' && this.canEdit) {
			e.preventDefault();
			void this.handleAddHint();
		}
	};

	toggleFullscreen = (element?: HTMLElement | null) => {
		if (!document.fullscreenElement) {
			element?.requestFullscreen();
		} else {
			document.exitFullscreen();
		}
	};

	setHintsContext = (imageId: string, canEdit: boolean) => {
		this.canEdit = canEdit;
		if (this.currentImageId === imageId) return;
		this.currentImageId = imageId;
		void this.loadHints(imageId);
	};

	private loadHints = async (imageId: string) => {
		const res = await getPanoramasImageHints(imageId);
		if (this.currentImageId !== imageId) return;
		if (res.status === 200 && res.data) {
			this.setTrees(res.data);
		} else {
			this.setTrees([]);
			showError(res.error?.description || 'Failed to load hints');
		}
	};

	handleAddHint = async () => {
		if (!this.canEdit || !this.currentImageId || this.isBusy) return;

		const newHint: PanoramaHint = { angle: this.yaw };
		this.setTrees([...this.trees, newHint]);

		this.isBusy = true;
		const res = await addPanoramaImageHint(this.currentImageId, this.yaw);

		if (res.error) {
			this.setTrees(this.trees.filter((t) => t !== newHint));
			this.isBusy = false;
			showError(res.error.description || 'Failed to add hint');
			return;
		}

		const hintsRes = await getPanoramasImageHints(this.currentImageId);
		if (hintsRes.status === 200 && hintsRes.data) {
			this.setTrees(hintsRes.data);
		}
		this.isBusy = false;

		panoBus.emit('reloadHints');
	};

	handleDeleteHints = async () => {
		if (!this.canEdit || !this.currentImageId || this.isBusy) return;

		this.isBusy = true;
		const res = await deleteImageHints(this.currentImageId);

		if (res.error) {
			this.isBusy = false;
			showError(res.error.description || 'Failed to delete hints');
			return;
		}

		// Only remove the manual hints, keep the auto-generated tree pointers
		// and sibling image pointers
		this.setTrees(this.trees.filter((t) => t.tree_id || t.image_id));

		this.isBusy = false;
		panoBus.emit('reloadHints');
	};

	handleToggleHidden = async () => {
		if (!this.currentImageId || this.isBusy) return;

		const newHidden = !this.isHidden;
		this.isHidden = newHidden;
		this.isBusy = true;
		const res = await updatePanoramaImage(this.currentImageId, newHidden);
		this.isBusy = false;

		if (res.error) {
			this.isHidden = !newHidden;
			showError(res.error.description || 'Failed to update image visibility');
			return;
		}

		panoBus.emit('reload');
	};

	handleTreeClick = (treeId: string) => {
		void goto(routes.mapPreview(treeId));
	};

	handleImageClick = (imageId: string) => {
		void goto(routes.panorama(imageId));
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
					// displayed at horizon level. Clicking it opens the tree preview.
					const treeId = trees[i].tree_id as string;
					this.viewer.addHotSpot({
						id,
						pitch: 0,
						yaw: trees[i].angle,
						type: 'info',
						cssClass: 'tree-marker-disc',
						createTooltipFunc: (div) => {
							const instance = mount(TreeIcon, { target: div });
							this.mountedIcons.set(id, instance);
						},
						clickHandlerFunc: () => this.handleTreeClick(treeId)
					});
				} else if (trees[i].image_id) {
					// Sibling image pointer: light-blue disc with a camera icon.
					// Clicking it opens the sibling panorama image.
					const imageId = trees[i].image_id as string;
					this.viewer.addHotSpot({
						id,
						pitch: -5,
						yaw: trees[i].angle,
						type: 'info',
						cssClass: 'image-marker-disc',
						createTooltipFunc: (div) => {
							const instance = mount(CameraIcon, { target: div });
							this.mountedIcons.set(id, instance);
						},
						clickHandlerFunc: () => this.handleImageClick(imageId)
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
