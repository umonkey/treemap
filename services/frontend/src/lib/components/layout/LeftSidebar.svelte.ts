import { getStats } from '$lib/api/stats';
import type { IStats } from '$lib/types';

class ComponentState {
	public stats = $state<IStats | null>(null);

	public async fetchStats() {
		const res = await getStats();
		if (res.data) {
			this.stats = res.data;
		}
	}

	public static componentState = new ComponentState();
}

export const componentState = ComponentState.componentState;
