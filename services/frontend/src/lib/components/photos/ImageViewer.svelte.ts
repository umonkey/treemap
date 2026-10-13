const MIN_SCALE = 1;
const MAX_SCALE = 5;
const DOUBLE_TAP_SCALE = 2;
const DOUBLE_TAP_MS = 300;
const TAP_MOVE_THRESHOLD = 10;

export class ImageViewerLogic {
	onClose: () => void = () => {};

	scale = $state(1);
	offsetX = $state(0);
	offsetY = $state(0);
	dragging = $state(false);
	canPan = $derived(this.scale > 1);

	private imageEl: HTMLImageElement | null = null;
	private pointers = new Map<number, { x: number; y: number }>();
	private lastPanX = 0;
	private lastPanY = 0;
	private moved = 0;
	private lastTapTime = 0;
	private pinchStartScale = 1;
	private pinchStartDistance = 0;
	private pinchStartMidX = 0;
	private pinchStartMidY = 0;
	private pinchStartOffsetX = 0;
	private pinchStartOffsetY = 0;

	bind = (onClose: () => void) => {
		this.onClose = onClose;
	};

	bindImage = (el: HTMLImageElement | null) => {
		this.imageEl = el;
	};

	handleKeydown = (event: KeyboardEvent) => {
		if (event.key === 'Escape') this.onClose();
	};

	reset = () => {
		this.scale = 1;
		this.offsetX = 0;
		this.offsetY = 0;
		this.dragging = false;
		this.pointers.clear();
		this.moved = 0;
		this.lastTapTime = 0;
	};

	handleResize = () => this.clampOffsets();

	handleWheel = (event: WheelEvent) => {
		event.preventDefault();
		const factor = Math.exp(-event.deltaY * 0.0015);
		this.zoomTo(this.scale * factor, event.clientX, event.clientY);
	};

	handleDblClick = (event: MouseEvent) => {
		this.toggleZoomAt(event.clientX, event.clientY);
	};

	handlePointerDown = (event: PointerEvent) => {
		(event.currentTarget as HTMLElement | null)?.setPointerCapture?.(event.pointerId);
		this.pointers.set(event.pointerId, { x: event.clientX, y: event.clientY });
		this.moved = 0;

		if (this.pointers.size === 1) {
			this.lastPanX = event.clientX;
			this.lastPanY = event.clientY;
			this.dragging = this.scale > 1;
		} else if (this.pointers.size === 2) {
			this.dragging = false;
			const mid = this.pointerMidpoint();
			this.pinchStartScale = this.scale;
			this.pinchStartDistance = this.pointerDistance();
			this.pinchStartMidX = mid.x;
			this.pinchStartMidY = mid.y;
			this.pinchStartOffsetX = this.offsetX;
			this.pinchStartOffsetY = this.offsetY;
		}
	};

	handlePointerMove = (event: PointerEvent) => {
		if (!this.pointers.has(event.pointerId)) return;
		this.pointers.set(event.pointerId, { x: event.clientX, y: event.clientY });

		if (this.pointers.size === 1 && this.scale > 1) {
			const dx = event.clientX - this.lastPanX;
			const dy = event.clientY - this.lastPanY;
			this.moved += Math.abs(dx) + Math.abs(dy);
			this.offsetX += dx;
			this.offsetY += dy;
			this.lastPanX = event.clientX;
			this.lastPanY = event.clientY;
			this.clampOffsets();
		} else if (this.pointers.size === 2) {
			this.moved = Infinity; // a pinch is never a tap
			this.updatePinch();
		}
	};

	handlePointerUp = (event: PointerEvent) => {
		const wasSingle = this.pointers.size === 1;
		this.pointers.delete(event.pointerId);

		if (this.pointers.size === 1) {
			const [p] = [...this.pointers.values()];
			this.lastPanX = p.x;
			this.lastPanY = p.y;
		}

		if (this.pointers.size > 0) return;

		this.dragging = false;

		if (event.pointerType === 'touch' && wasSingle && this.moved < TAP_MOVE_THRESHOLD) {
			const now = performance.now();
			if (now - this.lastTapTime < DOUBLE_TAP_MS) {
				this.lastTapTime = 0;
				this.toggleZoomAt(event.clientX, event.clientY);
			} else {
				this.lastTapTime = now;
			}
		}
	};

	private clamp = (value: number, min: number, max: number) => Math.min(Math.max(value, min), max);

	private clampOffsets = () => {
		const vw = window.innerWidth;
		const vh = window.innerHeight;
		const w = (this.imageEl?.clientWidth ?? 0) * this.scale;
		const h = (this.imageEl?.clientHeight ?? 0) * this.scale;
		const maxX = Math.max(0, (w - vw) / 2);
		const maxY = Math.max(0, (h - vh) / 2);
		this.offsetX = this.clamp(this.offsetX, -maxX, maxX);
		this.offsetY = this.clamp(this.offsetY, -maxY, maxY);
	};

	private zoomTo = (targetScale: number, focalX: number, focalY: number) => {
		const s = this.scale;
		const next = this.clamp(targetScale, MIN_SCALE, MAX_SCALE);
		const cx = window.innerWidth / 2;
		const cy = window.innerHeight / 2;
		const px = (focalX - cx - this.offsetX) / s;
		const py = (focalY - cy - this.offsetY) / s;
		this.scale = next;
		this.offsetX += (s - next) * px;
		this.offsetY += (s - next) * py;
		this.clampOffsets();
	};

	private toggleZoomAt = (x: number, y: number) => {
		if (this.scale > 1) this.reset();
		else this.zoomTo(DOUBLE_TAP_SCALE, x, y);
	};

	private updatePinch = () => {
		const dist = this.pointerDistance();
		const mid = this.pointerMidpoint();
		const next = this.clamp(
			this.pinchStartScale * (dist / this.pinchStartDistance || 1),
			MIN_SCALE,
			MAX_SCALE
		);
		const cx = window.innerWidth / 2;
		const cy = window.innerHeight / 2;
		// Local point under the pinch-start midpoint, in pre-scale image coordinates.
		const px = (this.pinchStartMidX - cx - this.pinchStartOffsetX) / this.pinchStartScale;
		const py = (this.pinchStartMidY - cy - this.pinchStartOffsetY) / this.pinchStartScale;
		// Keep that local point under the *current* midpoint (zoom + pan together).
		this.scale = next;
		this.offsetX = mid.x - cx - next * px;
		this.offsetY = mid.y - cy - next * py;
		this.clampOffsets();
	};

	private pointerDistance = () => {
		const [a, b] = [...this.pointers.values()];
		return Math.hypot(a.x - b.x, a.y - b.y);
	};

	private pointerMidpoint = () => {
		const pts = [...this.pointers.values()];
		const x = pts.reduce((sum, p) => sum + p.x, 0) / pts.length;
		const y = pts.reduce((sum, p) => sum + p.y, 0) / pts.length;
		return { x, y };
	};
}
