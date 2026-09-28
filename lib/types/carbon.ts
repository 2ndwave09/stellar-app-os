export type ProjectType =
  | 'Reforestation'
  | 'Renewable Energy'
  | 'Mangrove Restoration'
  | 'Sustainable Agriculture'
  | 'Other';

/** Minimum quantity (in tonnes) required for a bulk-purchase order. */
export const BULK_PURCHASE_MIN_QUANTITY = 100;

/** Where to store corporate metadata for a bulk purchase. */
export type MetadataStorageType = 'none' | 'on-chain' | 'ipfs';

export type VerificationStatus =
  | 'Gold Standard'
  | 'Verra (VCS)'
  | 'Climate Action Reserve'
  | 'Plan Vivo'
  | 'Pending';

export interface ProjectCoordinates {
  latitude: number;
  longitude: number;
}

export interface CarbonProject {
  id: string;
  name: string;
  description: string;
  vintageYear: number;
  pricePerTon: number;
  availableSupply: number;
  isOutOfStock: boolean;
  type: ProjectType;
  location: string;
  coordinates: ProjectCoordinates;
  coBenefits: string[];
  verificationStatus: VerificationStatus;
}

export interface CreditSelectionState {
  projectId: string | null;
  quantity: number;
  calculatedPrice: number;
}

export interface CreditSelectionProps {
  projects: CarbonProject[];
  onSelectionChange?: (selection: CreditSelectionState) => void;
}

// ─── Bulk purchase ────────────────────────────────────────────────────────────

export interface CorporateMetadata {
  companyName?: string;
  registrationNumber?: string;
  contactEmail?: string;
  notes?: string;
  initiativeDescription?: string;
  initiativeUrl?: string;
  storageType?: MetadataStorageType;
}

export interface BulkPurchaseOrder {
  projectId: string;
  quantity: number;
  totalPrice: number;
  buyerPublicKey: string;
  network: 'testnet' | 'mainnet';
  metadata?: CorporateMetadata;
}

export interface BulkPurchaseResult {
  transactionXdr: string;
  networkPassphrase: string;
  fee: string;
  sequence: string;
  memoValue?: string;
  ipfsCid?: string;
}
