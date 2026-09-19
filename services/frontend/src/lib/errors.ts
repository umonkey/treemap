import { toast } from 'svelte-sonner';

export function showError(message: string) {
	toast.error(message);
}

export function showInfo(message: string) {
	toast.info(message);
}

export function showWarning(message: string) {
	toast.warning(message);
}
