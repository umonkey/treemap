import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';

import { SelectLocationDialog } from "./SelectLocationDialog";
import { ILatLng } from "@/types";

const POSITION = {
  lat: 56.26,
  lon: 28.48,
} as ILatLng;

const meta = {
  title: 'Dialogs/SelectLocationDialog',
  component: SelectLocationDialog,
  parameters: {
    layout: 'padded', // centered, fullscreen
  },
  args: {
    onContinue: fn(),
  },
} satisfies Meta<typeof SelectLocationDialog>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    position: POSITION,
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    position: POSITION,
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
