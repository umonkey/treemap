import { mapState } from './MapLibre.svelte.ts';

export function mapBottomPadding(node: HTMLElement) {
	const owner = Symbol('mapBottomPadding');
	const mq = window.matchMedia('(max-width: 1023px)');

	const update = () => {
		mapState.setBottomPadding(mq.matches ? node.clientHeight : 0, owner);
	};

	const ro = new ResizeObserver(update);
	ro.observe(node);
	mq.addEventListener('change', update);
	update();

	return {
		destroy() {
			ro.disconnect();
			mq.removeEventListener('change', update);
			mapState.clearBottomPadding(owner);
		}
	};
}
