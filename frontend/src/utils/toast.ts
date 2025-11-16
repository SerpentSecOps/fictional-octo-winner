import toast from 'react-hot-toast';

export const showSuccess = (message: string) => {
  toast.success(message, {
    duration: 3000,
    position: 'bottom-right',
  });
};

export const showError = (message: string) => {
  toast.error(message, {
    duration: 4000,
    position: 'bottom-right',
  });
};

export const showLoading = (message: string) => {
  return toast.loading(message, {
    position: 'bottom-right',
  });
};

export const showInfo = (message: string) => {
  toast(message, {
    duration: 3000,
    position: 'bottom-right',
    icon: 'ℹ️',
  });
};

export const dismissToast = (toastId: string) => {
  toast.dismiss(toastId);
};
