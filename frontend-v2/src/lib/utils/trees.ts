import type { ITree } from '$lib/types';

export const shortDetails = (tree: ITree): string => {
	const parts = [];

	if (tree.height) {
		parts.push(`H=${tree.height.toFixed(1)}m`);
	} else {
		parts.push('H=?');
	}

	if (tree.diameter) {
		parts.push(`D=${tree.diameter.toFixed(1)}m`);
	} else {
		parts.push('D=?');
	}

	if (tree.circumference) {
		parts.push(`C=${(tree.circumference * 100).toFixed(1)}cm`);
	} else {
		parts.push('C=?');
	}

	parts.push('·');
	parts.push(tree.state);

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
	if (value === undefined || value === null) {
		return '???';
	}

	return `${value.toFixed(1)} m`;
};

export const formatCentimeters = (value: number | undefined | null): string => {
	if (value === undefined || value === null) {
		return '???';
	}

	return `${Math.round(value * 100)} cm`;
};
