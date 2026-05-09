import type { ReactNode } from "react";
import { cn } from "../../lib/cn";

type TabItem<T extends string> = {
  key: T;
  label: string;
};

type TabsProps<T extends string> = {
  items: Array<TabItem<T>>;
  active: T;
  onChange: (value: T) => void;
  className?: string;
};

export function Tabs<T extends string>({ items, active, onChange, className }: TabsProps<T>) {
  return (
    <div className={cn("tabs", className)}>
      {items.map((item) => (
        <button
          key={item.key}
          className={cn("tab", active === item.key && "active")}
          onClick={() => onChange(item.key)}
        >
          {item.label}
        </button>
      ))}
    </div>
  );
}

type TabsPanelProps = {
  children: ReactNode;
};

export function TabsPanel({ children }: TabsPanelProps) {
  return <>{children}</>;
}
