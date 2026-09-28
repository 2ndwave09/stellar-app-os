/**
 * GET /api/v2/marketplace/financing — Issue #1352
 * POST /api/v2/marketplace/financing — Issue #1352
 */

import { NextResponse } from 'next/server';
import { getFarmerCreditLines, applyForLandPrepCredit } from '@/lib/marketplace/farmerFinancing';

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const farmerId = searchParams.get('farmerId') || undefined;
    const lines = getFarmerCreditLines(farmerId);
    return NextResponse.json({ success: true, creditLines: lines });
  } catch (error: any) {
    return NextResponse.json({ success: false, error: error.message || 'Failed to fetch credit lines' }, { status: 500 });
  }
}

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { farmerId, farmerName, projectName, location, requestedAmount, carbonProjectLinkedId } = body;

    if (!farmerId || !projectName || !requestedAmount) {
      return NextResponse.json(
        { success: false, error: 'Missing required fields: farmerId, projectName, requestedAmount' },
        { status: 400 }
      );
    }

    const newLine = applyForLandPrepCredit({
      farmerId,
      farmerName: farmerName || 'Verified Farmer',
      projectName,
      location: location || 'Sub-Saharan Africa',
      requestedAmount: Number(requestedAmount),
      carbonProjectLinkedId: carbonProjectLinkedId || 'listing-001',
    });

    return NextResponse.json({ success: true, creditLine: newLine }, { status: 201 });
  } catch (error: any) {
    return NextResponse.json({ success: false, error: error.message || 'Failed to apply for land prep credit' }, { status: 500 });
  }
}
