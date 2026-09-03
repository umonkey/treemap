import { routes, goto } from '$lib/routes';
import type { ILatLng } from '$lib/types';
import { ls } from '$lib/utils/localStorage';

export class RangeSetupState {
	gcps = $state<Array<ILatLng | null>>([null, null, null, null]);

	constructor() {
		const saved = ls.read('range_tool_gcps');
		if (Array.isArray(saved)) {
			this.gcps = [0, 1, 2, 3].map((i) => {
				const g = saved[i];
				if (!g || Number.isNaN(g.lat) || Number.isNaN(g.lng) || (g.lat === 0 && g.lng === 0)) {
					return null;
				}
				return g;
			});
		}
	}

	setGcp = (index: number, val: ILatLng) => {
		const next = [...this.gcps];
		next[index] = val;
		this.gcps = next;
		ls.write('range_tool_gcps', this.gcps);
	};

	clearGcp = (index: number) => {
		const next = [...this.gcps];
		next[index] = null;
		this.gcps = next;
		ls.write('range_tool_gcps', this.gcps);
	};

	validCount = $derived(
		this.gcps.filter(
			(g) => g && !Number.isNaN(g.lat) && !Number.isNaN(g.lng) && !(g.lat === 0 && g.lng === 0)
		).length
	);
	canContinue = $derived(this.validCount >= 2);

	handleContinue = () => {
		if (this.canContinue) {
			goto(routes.toolsRangeEnter());
		}
	};
}
