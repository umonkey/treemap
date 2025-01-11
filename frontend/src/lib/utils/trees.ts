import type { ITree } from '$lib/types';
import { locale } from '$lib/locale';

export const formatYear = (value: number | null): string => {
	if (!value) {
		return 'no data';
	}

	return value;
};

export const shortDetails = (tree: ITree): string => {
	const parts = [];

	if (tree.height) {
		parts.push(`H=${formatMeters(tree.height)}`);
	} else {
		parts.push('H=?');
	}

	if (tree.diameter) {
		parts.push(`D=${formatMeters(tree.diameter)}`);
	} else {
		parts.push('D=?');
	}

	if (tree.circumference) {
		parts.push(`C=${formatCentimeters(tree.circumference)}`);
	} else {
		parts.push('C=?');
	}

	parts.push('·');
	parts.push(formatState(tree.state));

	return parts.join(' ');
};

export const formatLinks = (
	tree: ITree
): {
	text: string;
	url: string;
}[] => {
	const parts = [];

	if (tree.species) {
		parts.push({
			text: 'Wikipedia',
			url: `https://en.wikipedia.org/wiki/${tree.species}`
		});
	}

	if (tree.osm_id) {
		parts.push({
			text: 'OSM',
			url: `https://www.openstreetmap.org/node/${tree.osm_id}`
		});
	}

	return parts;
};

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

	return locale.centimeters(Math.round(value * 100));
};

export const formatSpecies = (value: string | null): string => {
	if (!value || value === 'Unknown') {
		return 'Unknown species';
	}

	return value;
};

export const formatState = (value: string | null): string => {
	if (!value || value === 'Unknown') {
		return 'Unknown species';
	}

	if (value === 'healthy') {
		return locale.stateHealthy();
	} else if (value === 'sick') {
		return locale.stateSick();
	} else if (value === 'dead') {
		return locale.stateDead();
	} else if (value === 'gone') {
		return locale.stateGone();
	} else if (value === 'stomp') {
		return locale.stateStomp();
	} else if (value === 'deformed') {
		return locale.stateDeformed();
	} else {
		return locale.stateUnknown();
	}
};
