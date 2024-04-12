import type { Meta, StoryObj } from '@storybook/react';

import { TreePopup } from "./TreePopup";
import { ITreeDetails } from "@/types";

const DETAILS = {
  id: "134025978690277376",
  lat: 40.180379,
  lon: 44.513434,
  name: "Great Oak",
  height: 25.0,
  circumference: 4.2,
  diameter: 15.0,
  state: "healthy",
  updated_at: 1712838375,
} as ITreeDetails;

const meta = {
  title: 'Maps/TreePopup',
  component: TreePopup,
  parameters: {
    layout: 'padded',
  },
  args: { },
} satisfies Meta<typeof TreePopup>;

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
