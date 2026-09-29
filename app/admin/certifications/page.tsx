import type { ReactNode } from 'react';
import { Text } from '@/components/atoms/Text';
import { CertificationRegistryPanel } from '@/components/organisms/CertificationRegistryPanel/CertificationRegistryPanel';
import { FarmerKycPanel } from '@/components/organisms/FarmerKycPanel/FarmerKycPanel';

export default function AdminCertificationsPage(): ReactNode {
  return (
    <div className="container mx-auto max-w-5xl px-4 py-8 sm:py-10">
      <div className="mb-8">
        <Text as="h1" variant="h2" className="mb-2">
          Certification registries
        </Text>
        <Text as="p" variant="muted">
          Pull project data from Verra and Gold Standard, verify credits, and manage renewal
          documentation.
        </Text>
      </div>

      <CertificationRegistryPanel />

      <div className="mt-12 mb-8">
        <Text as="h2" variant="h2" className="mb-2">
          Farmer KYC verification
        </Text>
        <Text as="p" variant="muted">
          Review identity verification, land ownership proof, agricultural experience, and
          certification eligibility screening for farmers.
        </Text>
      </div>

      <FarmerKycPanel />
    </div>
  );
}
