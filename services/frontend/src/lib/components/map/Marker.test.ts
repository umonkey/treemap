import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { MarkerLogic } from './Marker.svelte.ts';
import { mapBus } from '$lib/buses/mapBus';

describe('MarkerLogic', () => {
	let logic: MarkerLogic;

	beforeEach(() => {
		logic = new MarkerLogic();
	});

	afterEach(() => {
		logic.destroy();
	});

	it('initializes with undefined pin and center', () => {
		expect(logic.pin).toBeUndefined();
		expect(logic.center).toBeUndefined();
	});

	it('updates pin and center when pin event is emitted', () => {
		const coords = { lat: 40.1792, lng: 44.4991 };
		mapBus.emit('pin', coords);

		expect(logic.pin).toEqual(coords);
		expect(logic.center).toEqual(coords);
	});

	it('clears pin and center when undefined is emitted', () => {
		const coords = { lat: 40.1792, lng: 44.4991 };
		mapBus.emit('pin', coords);
		expect(logic.pin).toEqual(coords);

		mapBus.emit('pin', undefined);
		expect(logic.pin).toBeUndefined();
		expect(logic.center).toBeUndefined();
	});

	it('stops listening after destroy', () => {
		const coords1 = { lat: 40.1792, lng: 44.4991 };
		const coords2 = { lat: 41.0, lng: 45.0 };

		mapBus.emit('pin', coords1);
		expect(logic.pin).toEqual(coords1);

		logic.destroy();

		mapBus.emit('pin', coords2);
		// Should not update after destroy
		expect(logic.pin).toEqual(coords1);
	});
});
