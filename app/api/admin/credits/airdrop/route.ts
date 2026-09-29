import { NextResponse } from 'next/server';
import { isAdminRequest } from '@/lib/auth/admin';
import { mockAdminUsers } from '@/lib/api/mock/adminUsers';
import type {
  AirdropRequest,
  AirdropPreview,
  AirdropResult,
  AirdropRecipient,
} from '@/lib/types/carbon';
import {
  processFarmerPayment,
  parseFarmerPaymentInput,
  type FarmerPaymentInput,
  type FarmerPaymentResult,
} from '@/lib/payments/farmer-payment';
import { getPool } from '@/lib/db/client';

// Farmer payment processing (v1) - multi-currency support is implemented in
// @/lib/payments/farmer-payment and exposed through the tree-survival admin
// endpoints. This route also accepts a batch of farmer payments via POST so admins
// can queue multi-currency payouts while executing an airdrop.

// Rate limiting configuration
const RATE_LIMIT_WINDOW_MS = 60 * 1000; // 1 minute
const RATE_LIMIT_MAX_REQUESTS = 100; // per window
const BASE_BACKOFF_MS = 1000;
const MAX_BACKOFF_MS = 5 * 60 * 1000; // 5 minutes

const requestTimestamps = new Map<string, number[]>();
const blockedUntil = new Map<string, number>();
const violationCount = new Map<string, number>();

function getClientKeys(request: Request): string[] {
  const keys: string[] = [];
  const ip = request.headers.get('x-forwarded-for')?.split(',')[0].trim();
  if (ip) keys.push(`ip:${ip}`);
  const apiKey = request.headers.get('x-api-key');
  if (apiKey) keys.push(`apiKey:${apiKey}`);
  if (keys.length === 0) keys.push('unknown');
  return keys;
}

function checkRateLimit(key: string): { allowed: boolean; retryAfter?: number } {
  const now = Date.now();
  const blockedUntilTime = blockedUntil.get(key) ?? 0;

  if (now < blockedUntilTime) {
    return { allowed: false, retryAfter: blockedUntilTime - now };
  }

  const timestamps = (requestTimestamps.get(key) ?? []).filter((ts) => now - ts < RATE_LIMIT_WINDOW_MS);

  if (timestamps.length >= RATE_LIMIT_MAX_REQUESTS) {
    // Calculate how long until the oldest request in the window expires
    const oldestTimestamp = timestamps[0];
    const retryAfter = Math.max(1, oldestTimestamp + RATE_LIMIT_WINDOW_MS - now);

    // Apply exponential backoff
    const violations = (violationCount.get(key) ?? 0) + 1;
    violationCount.set(key, violations);
    const backoffMs = Math.min(BASE_BACKOFF_MS * Math.pow(2, violations - 1), MAX_BACKOFF_MS);
    blockedUntil.set(key, now + Math.max(retryAfter, backoffMs));

    return { allowed: false, retryAfter: Math.max(retryAfter, backoffMs) };
  }

  // Allow request and record it
  timestamps.push(now);
  requestTimestamps.set(key, timestamps);
  // Reset violation count on successful request
  violationCount.set(key, 0);
  return { allowed: true };
}

function enforceRateLimit(request: Request): NextResponse | null {
  const keys = getClientKeys(request);
  for (const key of keys) {
    const result = checkRateLimit(key);
    if (!result.allowed) {
      return NextResponse.json(
        { error: 'Too many requests, please slow down.' },
        { status: 429, headers: { 'Retry-After': String(Math.ceil((result.retryAfter ?? 0) / 1000)) } }
      );
    }
  }
  return null;
}

// Audit logging helper
function logAudit(action: string, details: Record<string, unknown>): void {
  const entry = {
    timestamp: new Date().toISOString(),
    action,
    ...details,
  };
  console.log(`[audit] ${JSON.stringify(entry)}`);
}

// Carbon credit fractionalization - retail access
// Minimum purchase is 1 ton instead of 100+ ton blocks.
const MINIMUM_PURCHASE_TONS = 1;
const MAX_FRACTIONAL_TORS = 1000000;

interface FractionalizationRequest {
  projectId: string;
  totalTons: number;
  minimumPurchaseTons?: number;
}

interface FractionalizationResult {
  projectId: string;
  totalTons: number;
  minimumPurchaseTons: number;
  availableUnits: number;
  status: 'queued' | 'failed';
  error?: string;
}

function validateFractionalization(
  request: FractionalizationRequest
): string | null {
  if (!request.projectId) return 'projectId is required';
  if (!request.totalTons || request.totalTons <= 0) {
    return 'totalTons must be greater than zero';
  }
  if (request.totalTons > MAX_FRACTIONAL_TORS) {
    return `totalTons exceeds maximum of ${MAX_FRACTIONAL_TORS}`;
  }
  const minimum = request.minimumPurchaseTons ?? MINIMUM_PURCHASE_TONS;
  if (minimum < MINIMUM_PURCHASE_TONS) {
    return `minimumPurchaseTons must be at least ${MINIMUM_PURCHASE_TONS} ton`;
  }
  if (minimum > request.totalTons) {
    return 'minimumPurchaseTons cannot exceed totalTons';
  }
  if (!Number.isInteger(minimum)) {
    return 'minimumPurchaseTons must be a whole number of tons';
  }
  return null;
}

function fractionalizeProject(
  request: FractionalizationRequest
): FractionalizationResult {
  const validationError = validateFractionalization(request);
  if (validationError) {
    return {
      projectId: request.projectId,
      totalTons: request.totalTons,
      minimumPurchaseTons: request.minimumPurchaseTons ?? MINIMUM_PURCHASE_TONS,
      availableUnits: 0,
      status: 'failed',
      error: validationError,
    };
  }

  const minimum = request.minimumPurchaseTons ?? MINIMUM_PURCHASE_TONS;
  const availableUnits = Math.floor(request.totalTons / minimum);

  // TODO: replace with real Stellar CARBON token minting for fractional units
  return {
    projectId: request.projectId,
    totalTons: request.totalTons,
    minimumPurchaseTons: minimum,
    availableUnits: availableUnits,
    status: 'queued',
  };
}

function getEarlySponsors(platformLaunchDate: string): AirdropRecipient[] {
  const launch = new Date(platformLaunchDate);
  const cutoff = new Date(launch);
  cutoff.setMonth(cutoff.getMonth() + 6);

  return mockAdminUsers
    .filter((user) => {
      if (user.status === 'Deleted') return false;
      const joined = new Date(user.joinedAt);
      if (joined < launch || joined > cutoff) return false;
      return user.activityLog.some(
        (entry) => entry.type === 'donation' || entry.type === 'credit_purchase'
      );
    })
    .map((user) => ({
      userId: user.id,
      walletAddress: user.walletAddress,
      email: user.email,
      joinedAt: user.joinedAt,
    }));
}

export async function GET(request: Request) {
  if (!(await isAdminRequest())) {
    logAudit('admin.airdrop.preview', { status: 'denied' });
    return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
  }

  const rateLimitInspection = enforceRateLimit(request);
  if (rateLimitInspection) {
    logAudit('admin.airdrop.preview', { status: 'rate_limited', keys: getClientKeys(request) });
    return rateLimitInspection;
  }

  const { searchParams } = new URL(request.url);
  const platformLaunchDate = searchParams.get('platformLaunchDate');
  const creditsPerSponsor = Number(searchParams.get('creditsPerSponsor') ?? 0);

  logAudit('admin.airdrop.preview', {
    status: 'started',
    platformLaunchDate,
    creditsPerSponsor,
  });

  if (!platformLaunchDate || isNaN(new Date(platformLaunchDate).getTime())) {
    logAudit('admin.airdrop.preview', { status: 'invalid_date' });
    return NextResponse.json({ error: 'Invalid or missing platformLaunchDate' }, { status: 400 });
  }

  if (creditsPerSponsor <= 0) {
    logAudit('admin.airdrop.preview', { status: 'invalid_credits' });
    return NextResponse.json(
      { error: 'creditsPerSponsor must be greater than zero' },
      { status: 400 }
    );
  }

  const recipients = getEarlySponsors(platformLaunchDate);
  const cutoff = new Date(platformLaunchDate);
  cutoff.setMonth(cutoff.getMonth() + 6);

  const preview: AirdropPreview = {
    recipients,
    totalCredits: recipients.length * creditsPerSponsor,
    cutoffDate: cutoff.toISOString(),
  };

  logAudit('admin.airdrop.preview', {
    status: 'success',
    recipientCount: recipients.length,
    totalCredits: preview.totalCredits,
  });

  return NextResponse.json(preview);
}

export async function POST(request: Request) {
  if (!(await isAdminRequest())) {
    logAudit('admin.airdrop.execute', { status: 'denied' });
    return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
  }

  const rateLimitInspection = enforceRateLimit(request);
  if (rateLimitInspection) {
    logAudit('admin.airdrop.execute', { status: 'rate_limited', keys: getClientKeys(request) });
    return rateLimitInspection;
  }

  try {
    const body = (await request.json()) as AirdropRequest & {
      farmerPayments?: unknown[];
    };
    const { creditsPerSponsor, projectId, platformLaunchDate } = body;

    logAudit('admin.airdrop.execute', {
      status: 'started',
      projectId,
      platformLaunchDate,
      creditsPerSponsor,
    });

    if (!projectId) {
      logAudit('admin.airdrop.execute', { status: 'missing_project_id' });
      return NextResponse.json({ error: 'projectId is required' }, { status: 400 });
    }

    if (!platformLaunchDate || isNaN(new Date(platformLaunchDate).getTime())) {
      logAudit('admin.airdrop.execute', { status: 'invalid_date' });
      return NextResponse.json({ error: 'Invalid or missing platformLaunchDate' }, { status: 400 });
    }

    if (!creditsPerSponsor || creditsPerSponsor <= 0) {
      logAudit('admin.airdrop.execute', { status: 'invalid_credits' });
      return NextResponse.json(
        { error: 'creditsPerSponsor must be greater than zero' },
        { status: 400 }
      );
    }

    const recipients = getEarlySponsors(platformLaunchDate);
    const cutoff = new Date(platformLaunchDate);
    cutoff.setMonth(cutoff.getMonth() + 6);

    const result: AirdropResult = {
      projectId,
      creditsPerSponsor,
      cutoffDate: cutoff.toISOString(),
      recipients,
      totalCredits: recipients.length * creditsPerSponsor,
      status: 'queued',
    };

    // Optional farmer payment batch attached to the airdrop execution.
    // This wires the multi-currency farmer payment path into the real
    // admin airdrop flow without adding a separate endpoint.
    const rawPayments = Array.isArray(body.farmerPayments) ? body.farmerPayments : [];
    const farmerPayments: FarmerPaymentResult[] = [];
    if (rawPayments.length > 0) {
      const pool = getPool();
      for (const rawPayment of rawPayments) {
        try {
          const input: FarmerPaymentInput = parseFarmerPaymentInput(rawPayment);
          const paymentResult = await processFarmerPayment(pool, input);
          farmerPayments.push(paymentResult);
        } catch (error) {
          const message = error instanceof Error ? error.message : 'Failed to process farmer payment';
          farmerPayments.push({
            paymentId: '',
            farmerId: '',
            amount: 0,
            currency: 'XLM' as FarmerPaymentResult['currency'],
            method: 'bank_transfer' as FarmerPaymentResult['method'],
            status: 'failed',
            reference: '',
            createdAt: new Date().toISOString(),
          });
          logAudit('admin.airdrop.farmer_payment.error', { status: 'error', error: message });
        }
      }
    }

    logAudit('admin.airdrop.execute', {
      status: 'success',
      recipientCount: recipients.length,
      totalCredits: result.totalCredits,
      farmerPayments: farmerPayments.length,
    });

    return NextResponse.json({
      ...result,
      farmerPayments,
    });
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Failed to execute airdrop';
    logAudit('admin.airdrop.execute', { status: 'error', error: message });
    return NextResponse.json({ error: message }, { status: 500 });
  }
}
