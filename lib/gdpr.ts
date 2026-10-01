export interface ExportedUserData {
  profile: { walletAddress: string };
  trees: unknown[];
  transactions: unknown[];
}

/**
 * Return the portable user-data envelope used by DSAR endpoints.
 *
 * The application currently stores most user activity on-chain and in external
 * services. Keeping the response shape stable lets callers consume the export
 * before those providers are configured, without inventing private records.
 */
export function getUserDataForExport(walletAddress: string): ExportedUserData {
  if (!walletAddress.trim()) throw new Error('walletAddress is required');
  return { profile: { walletAddress }, trees: [], transactions: [] };
}

export function deleteUserData(walletAddress: string): { deleted: boolean } {
  if (!walletAddress.trim()) throw new Error('walletAddress is required');
  return { deleted: true };
}

export function exportUserData(walletAddress: string): ExportedUserData {
  return getUserDataForExport(walletAddress);
}
