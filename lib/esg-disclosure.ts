/**
 * Buyer CSR / ESG disclosure helpers — Issue #1317
 *
 * Builds a shareable ESG disclosure from buyer carbon-offset data so
 * companies can send a report to investors and stakeholders.
 */

import type { BuyerAnalyticsSummary } from '@/lib/api/buyer-analytics';

export type EsgOffsetLine = {
  projectName: string;
  creditType: string;
  tonnesCo2e: number;
  verification: string;
};

export type EsgDisclosureInput = {
  companyName: string;
  period: string;
  totalTrees: number;
  totalCo2Offset: number;
  projectsSupported: string[];
  offsets?: EsgOffsetLine[];
};

export type EsgDisclosureReport = EsgDisclosureInput & {
  reportId: string;
  generatedAt: string;
  sharePath: string;
};

const SAMPLE_OFFSETS: EsgOffsetLine[] = [
  {
    projectName: 'Amazon Rainforest Restoration',
    creditType: 'ARR',
    tonnesCo2e: 180.4,
    verification: 'Verra VCS',
  },
  {
    projectName: 'Kenya Mangrove Planting',
    creditType: 'Blue carbon',
    tonnesCo2e: 142.1,
    verification: 'Gold Standard',
  },
  {
    projectName: 'Indonesia Peatland Protection',
    creditType: 'REDD+',
    tonnesCo2e: 127.7,
    verification: 'Verra VCS',
  },
];

export function defaultEsgDisclosure(): EsgDisclosureInput {
  return {
    companyName: 'Acme Corp',
    period: 'Q1 2026',
    totalTrees: 12_500,
    totalCo2Offset: 450.2,
    projectsSupported: SAMPLE_OFFSETS.map((line) => line.projectName),
    offsets: SAMPLE_OFFSETS,
  };
}

export function buildEsgReportId(companyName: string, period: string): string {
  const slug = companyName
    .trim()
    .toUpperCase()
    .replace(/[^A-Z0-9]+/g, '-')
    .replace(/^-|-$/g, '')
    .slice(0, 24);
  const periodSlug = period
    .trim()
    .toUpperCase()
    .replace(/[^A-Z0-9]+/g, '');
  return `ESG-${periodSlug || 'PERIOD'}-${slug || 'BUYER'}`;
}

export function buildEsgSharePath(input: EsgDisclosureInput): string {
  const params = new URLSearchParams({
    company: input.companyName,
    period: input.period,
    trees: String(input.totalTrees),
    co2: String(input.totalCo2Offset),
    projects: input.projectsSupported.join('|'),
  });
  if (input.offsets?.length) params.set('offsets', JSON.stringify(input.offsets));
  return `/esg-disclosure?${params.toString()}`;
}

export function parseEsgShareParams(params: URLSearchParams): EsgDisclosureInput {
  const defaults = defaultEsgDisclosure();
  const projects = params.get('projects');
  const trees = Number(params.get('trees'));
  const co2 = Number(params.get('co2'));

  return {
    companyName: params.get('company')?.trim() || defaults.companyName,
    period: params.get('period')?.trim() || defaults.period,
    totalTrees: Number.isFinite(trees) && trees >= 0 ? trees : defaults.totalTrees,
    totalCo2Offset: Number.isFinite(co2) && co2 >= 0 ? co2 : defaults.totalCo2Offset,
    projectsSupported: projects
      ? projects
          .split('|')
          .map((name) => name.trim())
          .filter(Boolean)
      : defaults.projectsSupported,
    offsets: parseEsgOffsetsParam(params.get('offsets')),
  };
}

/** Reads the `offsets` share param, dropping anything that is not a valid line. */
export function parseEsgOffsetsParam(raw: string | null): EsgOffsetLine[] {
  if (!raw) return [];
  try {
    const parsed: unknown = JSON.parse(raw);
    if (!Array.isArray(parsed)) return [];
    return parsed.flatMap((item): EsgOffsetLine[] => {
      if (typeof item !== 'object' || item === null) return [];
      const line = item as Record<string, unknown>;
      const tonnes = Number(line.tonnesCo2e);
      if (
        typeof line.projectName !== 'string' ||
        typeof line.creditType !== 'string' ||
        typeof line.verification !== 'string' ||
        !Number.isFinite(tonnes) ||
        tonnes < 0
      ) {
        return [];
      }
      return [
        {
          projectName: line.projectName,
          creditType: line.creditType,
          tonnesCo2e: tonnes,
          verification: line.verification,
        },
      ];
    });
  } catch {
    return [];
  }
}

/**
 * 1 TREE token = 48 kg CO2 (CO2_KG_PER_TREE in lib/stellar/tree-asset.ts).
 * Mirrored here, as lib/api/ghg-protocol.ts does, so client code does not
 * import the Stellar SDK.
 */
const CO2_KG_PER_TREE = 48;

const PLATFORM_LABELS: Record<string, string> = {
  stellar: 'Stellar on-chain',
  'tree-registry': 'Tree registry',
  verra: 'Verra VCS',
  'gold-standard': 'Gold Standard',
  'climate-action-reserve': 'Climate Action Reserve',
  'plan-vivo': 'Plan Vivo',
  unverified: 'Unverified',
};

const round1 = (value: number): number => Math.round(value * 10) / 10;

/**
 * Maps a buyer-analytics summary (real offset purchases) onto ESG disclosure
 * input. Trees supported is derived from sequestration tonnes.
 */
export function buildEsgInputFromAnalytics(
  summary: BuyerAnalyticsSummary,
  meta: { companyName: string; period: string }
): EsgDisclosureInput {
  const offsets: EsgOffsetLine[] = summary.supplyChain.map((project) => ({
    projectName: project.projectName,
    creditType:
      project.assetType === 'sequestration'
        ? 'Tree sequestration'
        : (project.projectType ?? 'Carbon credit'),
    tonnesCo2e: round1(project.tonnes),
    verification: PLATFORM_LABELS[project.platform] ?? project.platform,
  }));

  return {
    companyName: meta.companyName,
    period: meta.period,
    totalTrees: Math.round((summary.totals.sequestrationTonnes * 1000) / CO2_KG_PER_TREE),
    totalCo2Offset: round1(summary.totals.totalTonnes),
    projectsSupported: offsets.map((line) => line.projectName),
    offsets,
  };
}

export function createEsgDisclosure(input: EsgDisclosureInput): EsgDisclosureReport {
  const companyName = input.companyName.trim() || 'Unnamed buyer';
  const period = input.period.trim() || 'Current period';
  const projectsSupported = input.projectsSupported.map((name) => name.trim()).filter(Boolean);
  const offsets = input.offsets ?? [];
  const report: EsgDisclosureReport = {
    companyName,
    period,
    totalTrees: Math.max(0, input.totalTrees),
    totalCo2Offset: Math.max(0, input.totalCo2Offset),
    projectsSupported,
    offsets,
    reportId: buildEsgReportId(companyName, period),
    generatedAt: new Date().toISOString(),
    sharePath: '',
  };
  report.sharePath = buildEsgSharePath(report);
  return report;
}
