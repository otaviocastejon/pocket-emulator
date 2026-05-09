import type { ButtonHTMLAttributes } from "react";
import { cn } from "../../lib/cn";

type ButtonVariant =
  | "primary"
  | "secondary"
  | "success"
  | "destructive"
  | "ghost"
  | "icon";

type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: ButtonVariant;
};

export function Button({ className, variant = "primary", ...props }: ButtonProps) {
  return (
    <button
      className={cn(
        "btn",
        variant === "secondary" && "btn-secondary",
        variant === "success" && "btn-success",
        variant === "destructive" && "btn-destructive",
        variant === "ghost" && "btn-ghost",
        variant === "icon" && "iconBtn",
        className,
      )}
      {...props}
    />
  );
}
