// Global imports.
import type { Preview } from "@storybook/react";
import { MemoryRouter } from "react-router-dom";

// Project imports.
import { IComment, ISpecies, ITreeInfo, ITreeDetails } from "@/types";
import {
  SAMPLE_COMMENTS,
  SAMPLE_DEFAULTS,
  SAMPLE_SPECIES_SEARCH,
  SAMPLE_TREE,
  SAMPLE_TREES,
  SAMPLE_USER1,
  SAMPLE_USER2,
} from "@/sample-data";

// Local imports.
import { StoryWrapper } from "./StoryWrapper";
import "./preview.css";
import "../src/index.css";

const preview: Preview = {
  parameters: {
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },

    // Configure viewports.
    // https://storybook.js.org/docs/essentials/viewport
    viewport: {
      viewports: {
        desktop: {
          name: "Desktop",
          styles: {
            width: "1600px",
            height: "900px",
          },
        },
        responsive: {
          name: "Responsive",
          styles: {
            width: "100%",
            height: "100%",
          },
        },
        mobile1: {
          name: "iPhone 5",
          styles: {
            width: "320px",
            height: "568px",
          },
        },
        mobile2: {
          name: "iPhone 14",
          styles: {
            width: "390px",
            height: "844px",
          },
        },
      },
    },

    mockAddonConfigs: {
      globalMockData: [
        {
          url: "/v1/trees?n=n&e=e&s=s&w=w",
          method: "GET",
          status: 200,
          response: {
            trees: SAMPLE_TREES,
          },
          delay: 1000,
        },
        {
          url: "/v1/trees?n=n&e=e&s=s&w=w&search=_",
          method: "GET",
          status: 200,
          response: {
            trees: SAMPLE_TREES,
          },
          delay: 1000,
        },
        {
          url: "/v1/trees",
          method: "POST",
          status: 200,
          response: {
            id: 12345,
            lat: 40.181389,
            lon: 44.514444,
            name: "New Tree",
          } as ITreeInfo,
          delay: 2000,
        },
        {
          url: "/v1/trees/defaults",
          method: "GET",
          status: 200,
          response: SAMPLE_DEFAULTS,
          delay: 2000,
        },
        {
          url: "/v1/trees/134793003121381380",
          method: "GET",
          status: 200,
          response: SAMPLE_TREE,
          delay: 500,
        },
        {
          url: "/v1/trees/134793003121381380",
          method: "PUT",
          status: 202,
          delay: 500,
        },
        {
          url: "/v1/trees/134793003121381380/position",
          method: "PUT",
          status: 202,
          delay: 500,
        },
        {
          url: "/v1/trees/134793003121381380/comments",
          method: "GET",
          status: 200,
          response: SAMPLE_COMMENTS,
          delay: 500,
        },

        {
          url: "/v1/species/search?query=query",
          method: "GET",
          status: 200,
          delay: 300,
          response: [
            {
              "name": "Fraxinus",
              "local": "Ash",
            },
            {
              "name": "Quecrus",
              "local": "Oak",
            },
            {
              "name": "Ulmus",
              "local": "Elm",
            },
          ] as ISpecies[],
        },
        {
          url: "/v1/species/search?query=_",
          method: "GET",
          status: 200,
          delay: 500,
          response: SAMPLE_SPECIES_SEARCH,
        },
        {
          url: "/v1/species/suggest",
          method: "GET",
          status: 200,
          delay: 500,
          response: ["Acer", "Ulmus", "Unknown tree"],
        },
        {
          url: "/v1/users/user1",
          method: "GET",
          status: 200,
          delay: 100,
          response: SAMPLE_USER1,
        },
        {
          url: "/v1/users/user2",
          method: "GET",
          status: 200,
          delay: 100,
          response: SAMPLE_USER2,
        },
      ],
    },
  },

  decorators: [
    (Story) => (
      <StoryWrapper>
        <MemoryRouter>
          <Story />
        </MemoryRouter>
      </StoryWrapper>
    ),
  ],
};

export default preview;
