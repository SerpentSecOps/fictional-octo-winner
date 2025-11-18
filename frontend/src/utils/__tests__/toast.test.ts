import { describe, it, expect, vi, beforeEach } from 'vitest';
import { showError, showSuccess } from '../toast';
import toast from 'react-hot-toast';

// Mock react-hot-toast
vi.mock('react-hot-toast', () => ({
  default: {
    error: vi.fn(),
    success: vi.fn(),
  },
}));

describe('Toast utilities', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('showError', () => {
    it('should call toast.error with the provided message', () => {
      const message = 'Test error message';
      showError(message);

      expect(toast.error).toHaveBeenCalledWith(message);
      expect(toast.error).toHaveBeenCalledTimes(1);
    });

    it('should handle empty messages', () => {
      showError('');
      expect(toast.error).toHaveBeenCalledWith('');
    });

    it('should handle long messages', () => {
      const longMessage = 'a'.repeat(1000);
      showError(longMessage);
      expect(toast.error).toHaveBeenCalledWith(longMessage);
    });
  });

  describe('showSuccess', () => {
    it('should call toast.success with the provided message', () => {
      const message = 'Test success message';
      showSuccess(message);

      expect(toast.success).toHaveBeenCalledWith(message);
      expect(toast.success).toHaveBeenCalledTimes(1);
    });

    it('should handle empty messages', () => {
      showSuccess('');
      expect(toast.success).toHaveBeenCalledWith('');
    });

    it('should handle multiple consecutive calls', () => {
      showSuccess('First');
      showSuccess('Second');
      showSuccess('Third');

      expect(toast.success).toHaveBeenCalledTimes(3);
    });
  });
});
