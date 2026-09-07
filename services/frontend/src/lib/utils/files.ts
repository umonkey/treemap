export const STORAGE_COST_PER_GB_MONTH = 0.023;

export const storage_cost = (bytes: number): string => {
	const gb = bytes / (1024 * 1024 * 1024);
	const cost = gb * STORAGE_COST_PER_GB_MONTH;
	return `$${cost.toFixed(2)}`;
};
