import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';

import { AddTreePage } from "./AddTreePage";
import { ILatLng, ITreeInfo } from "@/types";

const CENTER = {
  lat: 40.181389,
  lon: 44.514444,
} as ILatLng;

const meta = {
  title: 'Pages/AddTreePage',
  component: AddTreePage,
  parameters: {
    layout: 'fullscreen', // centered
    mockData: [
      {
        url: "/v1/trees",
        method: "POST",
        status: 200,
        response: {
          id: "1",
          lat: 40.181389,
          lon: 44.514444,
          name: "Oak",
        } as ITreeInfo,
      },
    ],
  },
  args: {
    onSuccess: fn(),
    onCancel: fn(),
  },
} satisfies Meta<typeof AddTreePage>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    lat: CENTER.lat,
    lon: CENTER.lon,
    token: "foo.bar.baz",
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    lat: CENTER.lat,
    lon: CENTER.lon,
    token: "foo.bar.baz",
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
