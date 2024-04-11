import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';

import { EditTreeDialog } from "./EditTreeDialog";
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
  title: 'Dialogs/EditTreeDialog',
  component: EditTreeDialog,
  parameters: {
    layout: 'padded', // centered, fullscreen
  },
  args: {
    onSave: fn(),
    onCancel: fn(),
  },
} satisfies Meta<typeof EditTreeDialog>;

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
