import { DEFAULT_TREE } from '$lib/constants';
import type { ITree } from '$lib/types';
import { describe, expect, it } from 'vitest';
import { measurementParts, shortDetails } from './trees';

describe('measurementParts', () => {
	it('should return all measurements when present', () => {
		const tree: ITree = { ...DEFAULT_TREE, height: 10, diameter: 8.5, circumference: 1.44 };

		expect(measurementParts(tree)).toEqual([
			{ kind: 'height', text: 'H=10 m' },
			{ kind: 'diameter', text: 'D=8.5 m' },
			{ kind: 'circumference', text: 'C=144 cm' }
		]);
	});

	it('should return question marks when all measurements are missing', () => {
		const tree: ITree = { ...DEFAULT_TREE, height: null, diameter: null, circumference: null };

		expect(measurementParts(tree)).toEqual([
			{ kind: 'height', text: 'H=?' },
			{ kind: 'diameter', text: 'D=?' },
			{ kind: 'circumference', text: 'C=?' }
		]);
	});

	it('should handle a mix of present and missing measurements', () => {
		const tree: ITree = { ...DEFAULT_TREE, height: 7, diameter: null, circumference: 2.5 };

		expect(measurementParts(tree)).toEqual([
			{ kind: 'height', text: 'H=7 m' },
			{ kind: 'diameter', text: 'D=?' },
			{ kind: 'circumference', text: 'C=250 cm' }
		]);
	});
});

describe('shortDetails', () => {
	it('should join measurement parts with spaces', () => {
		const tree: ITree = { ...DEFAULT_TREE, height: 10, diameter: 8.5, circumference: 1.44 };

		expect(shortDetails(tree)).toBe('H=10 m D=8.5 m C=144 cm');
	});

	it('should return question marks when all measurements are missing', () => {
		const tree: ITree = { ...DEFAULT_TREE, height: null, diameter: null, circumference: null };

		expect(shortDetails(tree)).toBe('H=? D=? C=?');
	});
});
