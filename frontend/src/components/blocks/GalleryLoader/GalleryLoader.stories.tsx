import type { Meta, StoryObj } from '@storybook/react';

import { SAMPLE_TREE } from "@/sample-data";

import { GalleryLoader } from "./GalleryLoader";

const meta = {
  title: 'Blocks/GalleryLoader',
  component: GalleryLoader,
  parameters: {
    layout: 'padded', // centered, fullscreen
    mockData: [
      {
        url: `/v1/trees/${SAMPLE_TREE.id}`,
        method: "GET",
        status: 200,
        delay: 1000,
        response: SAMPLE_TREE,
      },
    ],
  },
} satisfies Meta<typeof GalleryLoader>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    id: "134793003121381380",
  },
};
