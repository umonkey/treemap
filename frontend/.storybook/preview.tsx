import type { Preview } from "@storybook/react";
import { MemoryRouter } from "react-router-dom";

import { StoryWrapper } from "./StoryWrapper";
import { ITreeInfo } from "@/types";

import "./preview.css";

const TREES = [
  {
    id: 1,
    lat: 40.181389,
    lon: 44.514444,
  },
  {
    id: 2,
    lat: 40.182389,
    lon: 44.515444,
  },
] as ITreeInfo[];

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
            trees: TREES,
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
