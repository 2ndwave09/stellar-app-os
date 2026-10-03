export const BULK_PURCHASE_MIN_QUANTITY = 1000;

export type MetadataStorageType = 'none' | 'on-chain' | 'ipfs';

export interface CorporateMetadata {
  storageType: MetadataStorageType;
  companyName: string;
  initiativeDescription: string;
  initiativeUrl?: string;
  /** IPFS CID — populated when storageType is 'ipfs' */
  storageRef?: string;
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
  ipfsCid?: string;
  memoValue?: string;
}

export type ProjectType =
  | 'Reforestation'
  | 'Renewable Energy'
  | 'Mangrove Restoration'
  | 'Sustainable Agriculture'
  | 'Other';

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
