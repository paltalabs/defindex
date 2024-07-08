import type { Meta, StoryObj } from "@storybook/react";

import { Card } from "../card";

const meta: Meta<typeof Card> = {
  component: Card,
};
export default meta;

type Story = StoryObj<typeof Card>;

export const Default: Story = {
  args: {
    children: "Card children",
    title: "Card title",
  },
};
