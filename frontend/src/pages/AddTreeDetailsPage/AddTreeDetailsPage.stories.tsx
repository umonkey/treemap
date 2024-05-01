import type { Meta, StoryObj } from '@storybook/react';

import { AddTreeDetailsPage } from "./AddTreeDetailsPage";
import { ILatLng, ITreeInfo } from "@/types";

const CENTER = {
  lat: 40.181389,
  lon: 44.514444,
} as ILatLng;

const meta = {
  title: 'Pages/AddTreeDetailsPage',
  component: AddTreeDetailsPage,
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
          species: "Oak",
        } as ITreeInfo,
      },
    ],
  },
} satisfies Meta<typeof AddTreeDetailsPage>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    lat: CENTER.lat,
    lon: CENTER.lon,
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};
