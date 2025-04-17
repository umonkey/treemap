import { apiClient } from "$lib/api";
import type { IResponse, IStateStats } from "$lib/types";
import { get } from "svelte/store";
import { describe, expect, it } from "vitest";
import { loadStateStats } from "./loadStateStats";

describe("hooks/loadStateStats", async () => {
	it("should load empty list", async () => {
		apiClient.getStateStats = async (): Promise<IResponse<IStateStats[]>> => {
			return {
				status: 200,
				data: [],
			};
		};

		const { loading, error, data, reload } = loadStateStats();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(error)).toBeUndefined();
		expect(get(data)).toEqual([]);
	});

	it("should load non-empty list", async () => {
		apiClient.getStateStats = async (): Promise<IResponse<IStateStats[]>> => {
			return {
				status: 200,
				data: [
					{
						state: "dead",
						count: 10,
					},
				],
			};
		};

		const { loading, error, data, reload } = loadStateStats();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(error)).toBeUndefined();
		expect(get(data).length).toEqual(1);
		expect(get(data)[0].state).toEqual("dead");
	});

	it("should return an error", async () => {
		apiClient.getStateStats = async (): Promise<IResponse<IStateStats[]>> => {
			return {
				status: 500,
				data: undefined,
				error: {
					code: "SomethingWentWrong",
					description: "something went wrong",
				},
			};
		};

		const { loading, error, data, reload } = loadStateStats();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(data)).toEqual([]);
		expect(get(error)?.description).toEqual("something went wrong");
	});
});
