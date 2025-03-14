import { apiClient } from "$lib/api";
import { DEFAULT_ME } from "$lib/constants";
import type { IMeResponse, IResponse } from "$lib/types";
import { get } from "svelte/store";
import { describe, expect, it } from "vitest";
import { loadMe } from "./loadMe";

describe("hooks/loadMe", async () => {
	it("should load a profile", async () => {
		apiClient.getMe = async (): Promise<IResponse<IMeResponse>> => {
			return {
				status: 200,
				data: DEFAULT_ME,
			};
		};

		const { loading, error, data, reload } = loadMe();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(error)).toBeUndefined();
		expect(get(data)?.name).toEqual(DEFAULT_ME.name);
	});

	it("should return an error", async () => {
		apiClient.getMe = async (): Promise<IResponse<IMeResponse>> => {
			return {
				status: 500,
				data: undefined,
				error: {
					code: "SomethingWentWrong",
					description: "something went wrong",
				},
			};
		};

		const { loading, error, data, reload } = loadMe();
		expect(get(loading)).toBe(true);

		await reload();

		expect(get(loading)).toBe(false);
		expect(get(data)).toBeUndefined();
		expect(get(error)?.description).toEqual("something went wrong");
	});
});
