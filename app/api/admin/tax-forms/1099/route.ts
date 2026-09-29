/**
 * Admin endpoint for farmer tax documentation — Issue #1300
 *
 * GET  /api/admin/tax-forms/1099
 * POST /api/admin/tax-forms/1099
 *
 * Provides admin access to 1099 tax forms. When called from the admin
 * analytics UI without parameters, defaults to the current tax year
 * and 1099-NEC CSV export.
 *
 * Closes #1300
 */

import { type NextRequest } from 'next/server';
import { GET as baseGET, POST as basePOST } from '@/app/api/tax-forms/1099/route';

export const runtime = 'nodejs';

export async function GET(request: NextRequest) {
  const url = new URL(request.url);
  const now = new Date();
  const defaultYear = String(now.getUTCFullYear());

  if (!url.searchParams.has('year')) {
    url.searchParams.set('year', defaultYear);
  }

  const acceptHeader = request.headers.get('accept') ?? '';
  if (!url.searchParams.has('format') && acceptHeader.includes('text/csv')) {
    url.searchParams.set('format', 'csv');
    if (!url.searchParams.has('type')) {
      url.searchParams.set('type', '1099-NEC');
    }
  }

  const modifiedRequest = new Request(url.toString(), {
    method: 'GET',
    headers: request.headers,
  }) as NextRequest;

  return baseGET(modifiedRequest);
}

export async function POST(request: NextRequest) {
  return basePOST(request);
}
