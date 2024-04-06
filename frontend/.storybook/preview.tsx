import type { Preview } from "@storybook/react";

import { StoryWrapper } from "./StoryWrapper";
import "./preview.css";

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
  },

  decorators: [
    (Story) => (
      <StoryWrapper>
        <Story />
      </StoryWrapper>
    ),
  ],
};

export default preview;
