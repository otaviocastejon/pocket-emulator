import type { HTMLAttributes } from "react";
import { cn } from "../../lib/cn";

export function Table({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return <div className={cn("table", className)} {...props} />;
}

export function TableRow({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return <div className={cn("row", className)} {...props} />;
}

export function TableHeadRow({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return <div className={cn("tableHead", "row", className)} {...props} />;
}
