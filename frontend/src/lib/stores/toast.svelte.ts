export type ToastVariant = 'success' | 'error' | 'info';

export interface ToastMessage {
  id: number;
  variant: ToastVariant;
  text: string;
}

class ToastStore {
  items = $state<ToastMessage[]>([]);
  private nextId = 1;

  push(variant: ToastVariant, text: string): void {
    const id = this.nextId++;
    this.items = [...this.items, { id, variant, text }];
    setTimeout(() => {
      this.dismiss(id);
    }, 4000);
  }

  dismiss(id: number): void {
    this.items = this.items.filter((item) => item.id !== id);
  }
}

export const toastStore = new ToastStore();
