import { mapLayerStore } from '$lib/stores/mapLayerStore';
import { get } from 'svelte/store';
import { locale } from './ToolSelector.lang';

export class ToolSelector {
	readonly tools = [
		{ id: 1, label: locale.toolShow(), hint: locale.hintShow() },
		{ id: 2, label: locale.toolHeight(), hint: locale.hintHeight() },
		{ id: 3, label: locale.toolDiameter(), hint: locale.hintDiameter() },
		{ id: 4, label: locale.toolCircumference(), hint: locale.hintCircumference() },
		{ id: 5, label: locale.toolChecklist(), hint: locale.hintChecklist() },
		{ id: 6, label: locale.toolCamera(), hint: locale.hintCamera() }
	];

	activeTool = $state<number>(1);

	activeHint = $derived(this.tools.find((tool) => tool.id === this.activeTool)?.hint ?? '');

	public init = () => {
		const layers = get(mapLayerStore);

		if (layers.missingHeight) this.activeTool = 2;
		else if (layers.missingDiameter) this.activeTool = 3;
		else if (layers.missingCircumference) this.activeTool = 4;
		else if (layers.missingObservations) this.activeTool = 5;
		else if (layers.missingPhotos) this.activeTool = 6;
		else this.activeTool = 1;
	};

	public selectTool = (id: number) => {
		this.activeTool = id;

		mapLayerStore.update((store) => {
			store.missingHeight = id === 2;
			store.missingDiameter = id === 3;
			store.missingCircumference = id === 4;
			store.missingObservations = id === 5;
			store.missingPhotos = id === 6;
			return store;
		});
	};
}
