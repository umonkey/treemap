import type { Meta, StoryObj } from '@storybook/react';

import { TreeDetails } from "./TreeDetails";
import { ITreeDetails } from "@/types";

const DETAILS = {
  id: "134793003121381380",
  lat: 40.181389,
  lon: 44.514444,
  name: "Sycamore",
  height: 18.5,
  circumference: 1.2,
  diameter: 7,
} as ITreeDetails;

const meta = {
  title: 'Blocks/TreeDetails',
  component: TreeDetails,
  parameters: {
    layout: 'padded', // centered, fullscreen
  },
} satisfies Meta<typeof TreeDetails>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    tree: DETAILS,
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    tree: DETAILS,
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
