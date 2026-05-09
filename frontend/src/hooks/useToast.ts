import { useCallback, useMemo, useRef, useState } from "react";

export type ToastTone = "success" | "info" | "warning" | "error";

export type ToastItem = {
  id: number;
  title: string;
  message?: string;
  tone: ToastTone;
};

type PushToastInput = {
  title: string;
  message?: string;
  tone?: ToastTone;
  ttlMs?: number;
};

export function useToast() {
  const [toasts, setToasts] = useState<ToastItem[]>([]);
  const nextId = useRef(1);

  const dismiss = useCallback((id: number) => {
    setToasts((curr) => curr.filter((t) => t.id !== id));
  }, []);

  const pushToast = useCallback(
    ({ title, message, tone = "info", ttlMs = 3200 }: PushToastInput) => {
      const id = nextId.current++;
      setToasts((curr) => [...curr, { id, title, message, tone }]);
      window.setTimeout(() => dismiss(id), ttlMs);
    },
    [dismiss],
  );

  return useMemo(
    () => ({
      toasts,
      pushToast,
      dismiss,
    }),
    [toasts, pushToast, dismiss],
  );
}
