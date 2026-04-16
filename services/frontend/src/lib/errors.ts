import { toast } from '@zerodevx/svelte-toast';

export function showError(message: string) {
	toast.push(message);
}
