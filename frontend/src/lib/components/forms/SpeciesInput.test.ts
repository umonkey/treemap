import { cleanup, render, screen, within } from "@testing-library/svelte";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";
import SpeciesInput from "./SpeciesInput.svelte";

beforeEach(() => {
	const mockFetch = vi.fn();

	mockFetch.mockImplementation(async (req, options) => {
		// Simulate history.
		if (req.url === "https://api.treemaps.app/v1/species/suggest") {
			return {
				ok: true,
				status: 200,
				json: async () => ["Ulmus", "Elm", "unknown"],
			};
		}

		// Simulate search suggestions.
		if (req.url.startsWith("https://api.treemaps.app/v1/species/search?")) {
			return {
				ok: true,
				status: 200,
				json: async () => [
					{
						name: "Ulmus",
						local: "Elm",
					},
				],
			};
		}

		console.warn(`[test] Failing an unexpected fetch call: ${req.url}`);

		throw new Error(`Unexpected fetch call: ${req.url}`);
	});

	vi.stubGlobal("fetch", mockFetch);
});

afterEach(() => {
	vi.unstubAllGlobals();
});

describe("SpeciesInput", async () => {
	afterEach(cleanup);

	test("handle input", async () => {
		const user = userEvent.setup();

		let value = "";

		render(SpeciesInput, {
			value,
			onChange: (v: string) => {
				value = v;
			},
		});

		const input = screen.getByRole("textbox");
		await user.type(input, "ulmus");
		await user.tab();

		expect(value).toBe("ulmus");
	});

	test("show suggestions", async () => {
		const user = userEvent.setup();

		let value = "";

		render(SpeciesInput, {
			value,
			onChange: (v: string) => {
				value = v;
			},
		});

		const input = screen.getByRole("textbox");
		await user.type(input, "ulm");
		// await user.tab();

		const list = screen.getByRole("list", {
			name: /suggestions/i,
		});

		const items = within(list).getAllByRole("listitem");
		const names = items.map((item) => item.textContent);
		expect(names).toStrictEqual(["Ulmus ~ Elm"]);
	});
});
