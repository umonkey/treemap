const unselect = () => {
	document.getSelection()?.empty();
};

export const longtap = (el: HTMLElement, callback: () => void, time = 500): void => {
	let timer: number | null = null;
	let fired = false;

	const cancel = () => {
		if (timer !== null) {
			window.clearTimeout(timer);
			timer = null;
		}
	};

	el.addEventListener(
		'touchstart',
		() => {
			cancel();
			fired = false;
			timer = window.setTimeout(() => {
				timer = null;
				fired = true;
				unselect();
				callback();
			}, time);
		},
		{ passive: false }
	);

	// A move means the user is scrolling, so the press is not a long tap.
	el.addEventListener('touchmove', cancel);

	// A cancel (e.g. the browser takes over the gesture) aborts the press.
	el.addEventListener('touchcancel', () => {
		cancel();
		fired = false;
	});

	el.addEventListener(
		'touchend',
		(e) => {
			cancel();
			// Suppress the synthetic click that follows a long press.
			if (fired) e.preventDefault();
		},
		{ passive: false }
	);

	// Suppress the native long-press menu on Android once we have handled it.
	el.addEventListener('contextmenu', (e) => {
		if (fired) e.preventDefault();
	});
};
