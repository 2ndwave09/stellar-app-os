import { NextResponse } from 'next/server';
import { getPool } from '@/lib/db/client';
import { isAdminRequest } from '@/lib/auth/admin';
import { getTreeAnalytics, parseTreeAnalyticsFilters } from '@/lib/analytics/tree-survival';
import { getCarbonOffsetEstimate, parseCarbonOffsetInput } from '@/lib/analytics/carbon-offset';
import { getBulkPurchaseAgreements, parseBulkPurchaseFilters, createBulkPurchaseAgreement, parseBulkPurchaseInput } from '@/lib/marketplace/bulk-purchase';

export const runtime = 'nodejs';

/**
 * GET /api/admin/analytics/tree-survival
 *
 * Returns survival rate, lifecycle counts, cost per tree, and sponsor retention
 * grouped independently by species, region, and planter team.
 */
export async function GET(request: Request): Promise<NextResponse> {
  if (!(await isAdminRequest())) {
    return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
  }
  try {
    const filters = parseTreeAnalyticsFilters(new URL(request.url).searchParams);
    const report = await getTreeAnalytics(getPool(), filters);
    return NextResponse.json(report, {
      headers: { 'Cache-Control': 'private, max-age=60, stale-while-revalidate=300' },
    });
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Failed to generate tree analytics';
    const status = /must be|valid ISO|before or equal/.test(message) ? 400 : 500;
    console.error('[tree-survival-analytics]', error);
    return NextResponse.json({ error: message }, { status });
  }
}

/**
 * GET /api/admin/analytics/tree-survival?bulk=true
 *
 * Returns bulk purchase agreements with negotiated volume discount tiers
 * for corporate buyers purchasing 100+ ton batches from farmers.
 */
export async function GET_BULK(request: Request): Promise<NextResponse> {
  if (!(await isAdminRequest())) {
    return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
  }
  try {
    const filters = parseBulkPurchaseFilters(new URL(request.url).searchParams);
    const agreements = await getBulkPurchaseAgreements(getPool(), filters);
    return NextResponse.json(agreements, {
      headers: { 'Cache-Control': 'private, max-age=60, stale-while-revalidate=300' },
    });
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Failed to fetch bulk purchase agreements';
    const status = /must be|valid ISO|before or equal|100\+ ton/.test(message) ? 400 : 500;
    console.error('[bulk-purchase-agreements]', error);
    return NextResponse.json({ error: message }, { status });
  }
}

/**
 * POST /api/admin/analytics/tree-survival
 *
 * Estimates the number of carbon credits an individual needs to offset their
 * annual emissions based on household size, car usage, and energy consumption.
 */
export async function POST(request: Request): Promise<NextResponse> {
  if (!(await isAdminRequest())) {
    return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
  }
  try {
    const input = parseCarbonOffsetInput(await request.json());
    const estimate = getCarbonOffsetEstimate(input);
    return NextResponse.json(estimate, {
      headers: { 'Cache-Control': 'private, max-age=60, stale-while-revalidate=300' },
    });
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Failed to estimate carbon offset';
    const status = /must be|required|invalid|non-negative/.test(message) ? 400 : 500;
    console.error('[carbon-offset-estimate]', error);
    return NextResponse.json({ error: message }, { status });
  }
}

/**
 * POST /api/admin/analytics/tree-survival?bulk=true
 *
 * Creates a bulk purchase agreement between a corporate buyer and a farmer
 * for 100+ ton batches at negotiated volume discount rates.
 */
export async function POST_BULK(request: Request): Promise<NextResponse> {
  if (!(await isAdminRequest())) {
    return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
  }
  try {
    const input = parseBulkPurchaseInput(await request.json());
    const agreement = await createBulkPurchaseAgreement(getPool(), input);
    return NextResponse.json(agreement, {
      headers: { 'Cache-Control': 'private, max-age=60, stale-while-revalidate=300' },
    });
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Failed to create bulk purchase agreement';
    const status = /must be|required|invalid|non-negative|100\+ ton/.test(message) ? 400 : 500;
    console.error('[bulk-purchase-agreement-create]', error);
    return NextResponse.json({ error: message }, { status });
  }
}
