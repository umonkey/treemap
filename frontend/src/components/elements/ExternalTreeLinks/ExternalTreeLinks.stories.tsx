import type { Meta, StoryObj } from '@storybook/react';

import { SAMPLE_TREES } from "@/sample-data";
import { ExternalTreeLinks } from "./ExternalTreeLinks";

const meta = {
  title: 'Elements/ExternalTreeLinks',
  component: ExternalTreeLinks,
  parameters: {
    layout: 'padded', // centered, fullscreen
  },
} satisfies Meta<typeof ExternalTreeLinks>;

export default meta;
type Story = StoryObj<typeof meta>;

export const WithOSM: Story = {
  args: {
    tree: {
      ...SAMPLE_TREES[0],
      osm_id: 12345,
    },
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const NoOSM: Story = {
  args: {
    tree: {
      ...SAMPLE_TREES[0],
      osm_id: null,
    },
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};
