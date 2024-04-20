import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';

import { SpeciesSelector } from "./SpeciesSelector";

const meta = {
  title: 'Elements/SpeciesSelector',
  component: SpeciesSelector,
  parameters: {
    layout: 'padded', // centered, fullscreen
  },
  args: {
    onChange: fn(),
  },
} satisfies Meta<typeof SpeciesSelector>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    value: "Chestnut",
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    value: "Chestnut",
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
