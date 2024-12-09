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
