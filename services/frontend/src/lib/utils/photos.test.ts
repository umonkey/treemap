import { describe, expect, it } from 'vitest';
import { findSpeciesPhotos } from './photos';

const CATALPA_NANA =
	'https://pepinierestaubin.ca/wp-content/uploads/2026/03/Catalpa-Bignonioides-Nana.jpg';

const ULMUS_GLABRA = [
	'https://www.vdberk.co.uk/media/88960/ulmus-glabra.jpg',
	'https://upload.wikimedia.org/wikipedia/commons/7/76/Ulmus_glabra.jpg'
];

describe('findSpeciesPhotos', () => {
	it('should match an exact species name', () => {
		expect(findSpeciesPhotos('catalpa bignonioides nana')).toEqual([CATALPA_NANA]);
	});

	it('should match case-insensitively', () => {
		expect(findSpeciesPhotos('Catalpa Bignonioides Nana')).toEqual([CATALPA_NANA]);
	});

	it('should normalize surrounding and collapsed whitespace', () => {
		expect(findSpeciesPhotos('  Catalpa   Bignonioides   Nana  ')).toEqual([CATALPA_NANA]);
	});

	it('should return all photos of a species', () => {
		expect(findSpeciesPhotos('ulmus glabra')).toEqual(ULMUS_GLABRA);
	});

	it('should return the same photos for every alias of a set', () => {
		expect(findSpeciesPhotos('platanus')).toEqual(findSpeciesPhotos('platanus orientalis'));
	});

	it('should not match an unlisted binomial', () => {
		expect(findSpeciesPhotos('catalpa bignonioides')).toEqual([]);
	});

	it('should return an empty list for unknown or empty species', () => {
		expect(findSpeciesPhotos('')).toEqual([]);
		expect(findSpeciesPhotos('unknown')).toEqual([]);
		expect(findSpeciesPhotos(null)).toEqual([]);
		expect(findSpeciesPhotos(undefined)).toEqual([]);
	});

	it('should return an empty list when no set matches', () => {
		expect(findSpeciesPhotos('Acer platanoides')).toEqual([]);
	});
});
