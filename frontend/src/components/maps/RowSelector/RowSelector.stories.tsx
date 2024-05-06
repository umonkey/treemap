// Global imports.
import type { Meta, StoryObj } from '@storybook/react';

// Project imports.
import { DEFAULT_MAP_CENTER, DEFAULT_MAP_ZOOM } from "@/utils/config";
import { MapBase } from "@/components";

// Local imports.
import { RowSelector } from "./RowSelector";

const meta = {
  title: 'Maps/RowSelector',
  component: RowSelector,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: { },
  render: (args) => (
    <MapBase center={DEFAULT_MAP_CENTER} zoom={DEFAULT_MAP_ZOOM}>
      <RowSelector {...args} />
    </MapBase>
  ),
} satisfies Meta<typeof RowSelector>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    center: DEFAULT_MAP_CENTER,
  },
};
