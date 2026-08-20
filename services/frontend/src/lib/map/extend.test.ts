import { describe, expect, it } from 'vitest';
import { extendBounds } from './extend';
import type { IBounds } from '$lib/types';

describe('extendBounds', () => {
	const bounds: IBounds = {
		n: 10,
		e: 20,
		s: 0,
		w: 0
	};

	it('should expand bounds with default factor 0.5', () => {
		const extended = extendBounds(bounds);
		expect(extended).toEqual({
			n: 15,
			e: 30,
			s: -5,
			w: -10
		});
	});

	it('should expand bounds with custom factor 1.0', () => {
		const extended = extendBounds(bounds, 1.0);
		expect(extended).toEqual({
			n: 20,
			e: 40,
			s: -10,
			w: -20
		});
	});

	it('should expand bounds with custom factor 2.0', () => {
		const extended = extendBounds(bounds, 2.0);
		expect(extended).toEqual({
			n: 30,
			e: 60,
			s: -20,
			w: -40
		});
	});

	it('should keep bounds unchanged with factor 0', () => {
		const extended = extendBounds(bounds, 0);
		expect(extended).toEqual(bounds);
	});
});
