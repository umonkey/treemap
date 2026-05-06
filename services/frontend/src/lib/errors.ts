import { toast } from 'svelte-sonner';

export function showError(message: string) {
	toast.error(message);
}
