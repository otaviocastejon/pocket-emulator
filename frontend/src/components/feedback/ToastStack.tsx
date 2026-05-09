import type { ToastItem } from "../../hooks/useToast";

type Props = {
  toasts: ToastItem[];
  onDismiss: (id: number) => void;
};

export function ToastStack({ toasts, onDismiss }: Props) {
  return (
    <div className="toastStack" aria-live="polite" aria-atomic="false">
      {toasts.map((toast) => (
        <div key={toast.id} className={`toast toast-${toast.tone}`}>
          <div className="toastBody">
            <div className="toastTitle">{toast.title}</div>
            {toast.message ? <div className="toastMessage">{toast.message}</div> : null}
          </div>
          <button className="toastClose" onClick={() => onDismiss(toast.id)} aria-label="Dismiss">
            ×
          </button>
        </div>
      ))}
    </div>
  );
}
