import MockAdapter from "axios-mock-adapter";
import { vi } from "vitest";

import { treeMapService } from "@/services/api";
import { ITreesResponse } from "@/services/api/TreeMapService";
import { ISpecies } from "@/types";

vi.mock("axios", async () => {
  const actual = await vi.importActual<typeof import("axios")>("axios");

  return {
    default: {
      ...actual.default,
      create: vi.fn().mockReturnThis(),
    }
  };
});

describe("TreeMapService", () => {
  let mock: MockAdapter;

  beforeEach(async () => {
    const actual = await vi.importActual<typeof import("axios")>("axios");

    mock = new MockAdapter(actual.default, {
      onNoMatch: "throwException",
    });

    mock.reset();
  });

  test("can get markers", async () => {
    mock.onGet("/v1/trees", {
      params: {
        n: 1.1,
        e: 2.2,
        s: 3.3,
        w: 4.4,
      },
    }).reply(200, {
      trees: [
        {
          id: "1",
          lat: 56.26,
          lon: 28.48,
          species: "Quercus",
        },
      ],
    } as ITreesResponse);

    const res = await treeMapService.getMarkers({
      north: 1.1,
      east: 2.2,
      south: 3.3,
      west: 4.4,
    });

    expect(res).toEqual([
      {
        id: "1",
        lat: 56.26,
        lon: 28.48,
        species: "Quercus",
      },
    ]);
  });

  test("submit a tree", async () => {
  });

  test("search for species", async () => {
    mock.onGet("/v1/species/search?query=oak").reply(200, [
      {
        "name": "Quercus",
        "local": "Oak",
      },
    ] as ISpecies[]);

    const res = await treeMapService.searchSpecies("oak");
    expect(res[0].name).toEqual("Quercus");
  });
});

