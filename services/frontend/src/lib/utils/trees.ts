import { locale } from '$lib/locale';
import type { ITree } from '$lib/types';

export const formatYear = (value: number | null): string => {
	if (!value) {
		return 'no data';
	}

	return value.toString();
};

export type MeasurementKind = 'height' | 'diameter' | 'circumference';

export type MeasurementPart = {
	kind: MeasurementKind;
	text: string;
};

export const measurementParts = (tree: ITree): MeasurementPart[] => [
	{ kind: 'height', text: `H=${tree.height ? formatMeters(tree.height) : '?'}` },
	{ kind: 'diameter', text: `D=${tree.diameter ? formatMeters(tree.diameter) : '?'}` },
	{
		kind: 'circumference',
		text: `C=${tree.circumference ? formatCentimeters(tree.circumference) : '?'}`
	}
];

export const shortDetails = (tree: ITree): string =>
	measurementParts(tree)
		.map((part) => part.text)
		.join(' ');

export const formatMeters = (value: number | undefined | null): string => {
	if (!value) {
		return '???';
	}

	let v = value.toFixed(1);

	if (v.endsWith('.0')) {
		v = v.slice(0, -2);
	}

	return locale.meters(v);
};

export const formatCentimeters = (value: number | undefined | null): string => {
	if (!value) {
		return '???';
	}

	return locale.centimeters(Math.round(value * 100).toString());
};

export const formatSpecies = (value: string | null): string => {
	if (!value || value === 'Unknown') {
		return 'Unknown species';
	}

	return value;
};

export const formatState = (value: string | null): string => {
	if (value === 'alive') {
		return locale.stateAlive();
	}

	if (value === 'error') {
		return locale.stateError();
	}

	if (value === 'dead') {
		return locale.stateDead();
	}

	if (value === 'gone') {
		return locale.stateGone();
	}

	if (value === 'stump') {
		return locale.stateStump();
	}

	if (value === 'replaced') {
		return locale.stateReplaced();
	}

	return locale.stateUnknown();
};
