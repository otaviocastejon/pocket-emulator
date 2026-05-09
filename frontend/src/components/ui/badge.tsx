import type { HTMLAttributes } from "react";
import { cn } from "../../lib/cn";

type BadgeVariant =
  | "platform"
  | "favorite"
  | "saved"
  | "unsaved"
  | "error"
  | "tip";

type BadgeProps = HTMLAttributes<HTMLSpanElement> & {
  variant?: BadgeVariant;
};

export function Badge({ className, variant = "platform", ...props }: BadgeProps) {
  return (
    <span
      className={cn(
        "badge",
        variant === "favorite" && "badge-favorite",
        variant === "saved" && "badge-saved",
        variant === "unsaved" && "badge-unsaved",
        variant === "error" && "badge-error",
        variant === "tip" && "badge-tip",
        className,
      )}
      {...props}
    />
  );
}
