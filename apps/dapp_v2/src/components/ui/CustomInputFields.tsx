import { createListCollection, Field, Input, Portal, Select } from "@chakra-ui/react";
import { basePadding } from "./Common";

interface ReusableSelectProps {
  collection: ReturnType<typeof createListCollection<{ label: string; value: string }>>;
  label: string;
  placeholder: string;
}

export function CustomSelect({ collection, label, placeholder }: ReusableSelectProps) {
  return (
    <Select.Root multiple collection={collection}>
      <Select.HiddenSelect />
      <Select.Label>{label}</Select.Label>
      <Select.Control>
        <Select.Trigger px={basePadding}>
          <Select.ValueText placeholder={placeholder} />
        </Select.Trigger>
        <Select.IndicatorGroup p={basePadding}>
          <Select.Indicator />
        </Select.IndicatorGroup>
      </Select.Control>
      <Portal>
        <Select.Positioner>
          <Select.Content>
            {collection.items.map((item) => (
              <Select.Item key={item.value} item={item}>
                {item.label}
                <Select.ItemIndicator />
              </Select.Item>
            ))}
          </Select.Content>
        </Select.Positioner>
      </Portal>
    </Select.Root>
  )
}

export function FormField({ label, placeholder, type }: { label: string; placeholder: string, type?: string }) {
  return (
    <Field.Root>
      <Field.Label>{label}</Field.Label>
      <Input type={type} placeholder={placeholder} px={basePadding} />
    </Field.Root>
  );
}

