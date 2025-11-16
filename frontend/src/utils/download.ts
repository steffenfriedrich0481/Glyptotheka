export const downloadUtils = {
  triggerDownload(blob: Blob, filename: string): void {
    const url = window.URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = filename;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    window.URL.revokeObjectURL(url);
  },

  async handleDownloadWithProgress(
    downloadFn: () => Promise<Blob>,
    filename: string,
    onProgress?: (progress: number) => void
  ): Promise<void> {
    try {
      if (onProgress) {
        onProgress(0);
      }
      
      const blob = await downloadFn();
      
      if (onProgress) {
        onProgress(100);
      }
      
      this.triggerDownload(blob, filename);
    } catch (error) {
      console.error('Download failed:', error);
      throw error;
    }
  },
};
