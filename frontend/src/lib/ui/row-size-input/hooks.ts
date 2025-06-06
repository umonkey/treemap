import { writable } from 'svelte/store';
import { locale } from '$lib/locale';

export const hooks = ({
	distance,
	value,
	onChange
}: {
	distance: number;
	value: number;
	onChange: (value: number) => void;
}) => {
	const hint = writable<string>();

	const updateHint = (count: number) => {
		const step = Math.round((distance / count) * 10) / 10;
		hint.set(locale.rowStepInfo(count, step));
		console.debug(`[map] Row count changed to ${count}, step=${step}`);
	};

	const handleChange = (value: number) => {
		updateHint(value);
		onChange(value);
	};

	updateHint(value);

	return { hint, handleChange };
};
