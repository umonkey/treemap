import { locale } from '$lib/locale';

class ComponentState {
	distance = $state<number>(0);
	value = $state<number>(0);
	onChange = $state<(value: number) => void>(() => {});

	hint = $derived.by(() => {
		const count = this.value;
		const step = Math.round((this.distance / count) * 10) / 10;
		return locale.rowStepInfo(count, step);
	});

	public handleChange = (value: number) => {
		this.value = value;
		console.debug(`[map] Row count changed to ${value}`);
		this.onChange(value);
	};
}

export const componentState = new ComponentState();
