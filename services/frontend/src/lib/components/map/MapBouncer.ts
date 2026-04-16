import type { LngLatBounds } from 'maplibre-gl';

export class MapBouncer {
	n: number = 0;
	e: number = 0;
	s: number = 0;
	w: number = 0;

	private differs = (current: LngLatBounds): boolean => {
		if (current.getNorth() != this.n) {
			return true;
		}

		if (current.getSouth() != this.s) {
			return true;
		}

		if (current.getWest() != this.w) {
			return true;
		}

		if (current.getEast() != this.e) {
			return true;
		}

		return false;
	};

	public changed = (current: LngLatBounds): boolean => {
		if (!this.differs(current)) {
			return false;
		}

		this.n = current.getNorth();
		this.e = current.getEast();
		this.s = current.getSouth();
		this.w = current.getWest();

		return true;
	};
}
