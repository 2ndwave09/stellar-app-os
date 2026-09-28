/**
 * Farmer Financing - Credit Lines for Land Prep (v1) — Issue #1352
 *
 * Provides financing for farmers to prepare land for carbon projects.
 * Repayment comes from first carbon credit sales. 0% interest.
 */

export interface LandPrepCreditLine {
  id: string;
  farmerId: string;
  farmerName: string;
  projectName: string;
  location: string;
  requestedAmount: number; // in USD
  approvedAmount: number; // in USD
  disbursedAmount: number;
  repaidAmount: number;
  status: 'Pending' | 'Active' | 'Repaid' | 'Defaulted';
  interestRate: number; // 0%
  repaymentTerms: string;
  carbonProjectLinkedId: string;
  createdAt: string;
}

export const mockLandPrepCreditLines: LandPrepCreditLine[] = [
  {
    id: 'cc-line-001',
    farmerId: 'farmer-kwame',
    farmerName: 'Kwame Mensah',
    projectName: 'Northern Ghana Agroforestry & Land Prep',
    location: 'Tamale, Northern Ghana',
    requestedAmount: 5000,
    approvedAmount: 5000,
    disbursedAmount: 5000,
    repaidAmount: 0,
    status: 'Active',
    interestRate: 0.0,
    repaymentTerms: 'Repaid automatically from first carbon credit sales (v1)',
    carbonProjectLinkedId: 'listing-005',
    createdAt: '2024-02-25T08:00:00Z',
  },
  {
    id: 'cc-line-002',
    farmerId: 'farmer-amina',
    farmerName: 'Amina Diallo',
    projectName: 'Sahel Regreening Initiative',
    location: 'Kano, Nigeria',
    requestedAmount: 3500,
    approvedAmount: 3500,
    disbursedAmount: 3500,
    repaidAmount: 3500,
    status: 'Repaid',
    interestRate: 0.0,
    repaymentTerms: 'Repaid from first carbon credit sales',
    carbonProjectLinkedId: 'listing-008',
    createdAt: '2024-01-10T09:30:00Z',
  },
];

export function getFarmerCreditLines(farmerId?: string): LandPrepCreditLine[] {
  if (!farmerId) return mockLandPrepCreditLines;
  return mockLandPrepCreditLines.filter((l) => l.farmerId === farmerId);
}

export function applyForLandPrepCredit(input: {
  farmerId: string;
  farmerName: string;
  projectName: string;
  location: string;
  requestedAmount: number;
  carbonProjectLinkedId: string;
}): LandPrepCreditLine {
  const newLine: LandPrepCreditLine = {
    id: `cc-line-${Date.now()}`,
    farmerId: input.farmerId,
    farmerName: input.farmerName,
    projectName: input.projectName,
    location: input.location,
    requestedAmount: input.requestedAmount,
    approvedAmount: input.requestedAmount, // 0% interest instant pre-approval for verified farmers v1
    disbursedAmount: input.requestedAmount,
    repaidAmount: 0,
    status: 'Active',
    interestRate: 0.0,
    repaymentTerms: '0% interest. Repaid from first carbon credit sales.',
    carbonProjectLinkedId: input.carbonProjectLinkedId,
    createdAt: new Date().toISOString(),
  };
  mockLandPrepCreditLines.unshift(newLine);
  return newLine;
}
